const std = @import("std");

const re = @cImport({
    @cDefine("PCRE2_CODE_UNIT_WIDTH", "8");
    @cInclude("pcre2.h");
});
const PCRE2_ZERO_TERMINATED = ~@as(re.PCRE2_SIZE, 0);

pub const IlliterateRef = struct {
    // Indent of the ref line (from the source)
    base_indent_match: []u8,
    // Is it an inline ref or an indented line
    is_inline: bool,
    // Name of the ref (ident)
    name: []u8,
    // Full ref_text to replace
    ref_text: []u8,
};

pub const IlliterateBlock = union(enum) {
    Named: struct {
        // Programming language
        lang: []u8,
        // Name of the block (ident)
        name: []u8,
        // Content of the block
        code_content: []u8,
        // Refs in the block
        refs_in_code: []IlliterateRef,
        // Params in the meta of the block
        params: std.StringHashMap([]u8),
    },
    File: struct {
        // Programming language
        lang: []u8,
        // Output path
        path: []u8,
        // Content of the block
        code_content: []u8,
        // Refs in the block
        refs_in_code: []IlliterateRef,
        // Params in the meta of the block
        params: std.StringHashMap([]u8),
    },
};

pub const IlliterateSourceFile = struct {
    // Path of the file
    path: []u8,
    // Code blocks in the file
    code_blocks: []IlliterateBlock,
};

// TODO: Remove shouldn't be required
pub const IlliterateCodeWithRefs = struct {
    code_content: []u8,
    refs_in_code: []IlliterateRef,
};

pub const IlliterateResolvedResult = struct {
    // Resolved refs
    resolved: std.StringHashMap([]u8),
    // Cyclic refs
    cyclic: [][]u8,
    // Missing refs
    missing: []struct { []u8, []u8 }
};

pub fn main() !void {
    var debug_allocator: std.heap.DebugAllocator(.{}) = .init;
    defer std.debug.assert(debug_allocator.deinit() == .ok);
    const gpa = debug_allocator.allocator();

    var threaded: std.Io.Threaded = .init(gpa, .{});
    defer threaded.deinit();

    const io = threaded.io();

    const content_dir_path = "examples/comment-blocks";
    var content_dir = try std.Io.Dir.cwd().openDir(io, content_dir_path, .{ .iterate = true });
    defer content_dir.close(io);

    var walker = try content_dir.walk(gpa);
    defer walker.deinit();

    const regex_code_block_pat =
        \\(?# Multiline regex)(?m)(?#Skip things in comments)(?n:<!--([^-]+|-++(?!>))*(-->)?(*SKIP)\0)(?#Skip things that aren't the start of a comment/block)|(?:[^`<]+(*SKIP)\0)|(?#Match the content)(?<backquotes>^````*+)(?<meta>.*)\n(?<content>(?:[^`]+|(?!\k<backquotes>)`+)*)\n\k<backquotes>
    ;
    const regex_code_block = try Regex.new(regex_code_block_pat);
    const regex_code_meta_pat =
        \\{\.(?<lang>[^\s}]+) ?(?<params>[^}]*)}
    ;
    const regex_code_meta = try Regex.new(regex_code_meta_pat);
    _ = regex_code_meta; // autofix
    const regex_meta_params_pat =
        \\(?<key>[[:alnum:]]+)=(?<value>[^\s]+)
    ;
    const regex_meta_params = try Regex.new(regex_meta_params_pat);
    _ = regex_meta_params; // autofix
    const regex_code_refs_pat =
        \\(?m)^(?:(?![\t ]+<<)(?<line_indent>[\t ]*+)(?:[^\n<]*+|<)+|(?<direct_indent>[\t ]*))(?<full_ref><<(?<ref>[^>]+)>>)
    ;
    const regex_code_refs = try Regex.new(regex_code_refs_pat);
    _ = regex_code_refs; // autofix

    while (try walker.next(io)) |entry| {
        if (std.mem.endsWith(u8, entry.basename, ".md")) {
            std.debug.print("--- {s}/{s} ---\n", .{content_dir_path, entry.path});

            const path = try std.Io.Dir.path.join(gpa, &[_][]const u8{ content_dir_path, entry.path });
            defer gpa.free(path);

            var file = try std.Io.Dir.cwd().openFile(io, path, .{});
            defer file.close(io);

            var read_buffer: [1024]u8 = undefined;
            var reader = file.reader(io, &read_buffer);
            const content = try reader.interface.allocRemaining(gpa, std.Io.Limit.unlimited);
            defer gpa.free(content);
            std.debug.print("{s}\n", .{ content });

            var matches = try regex_code_block.matches(gpa, content);
            defer matches.deinit();

            var matches_iterator = matches.iterator();
            while (matches_iterator.next()) |match_entry| {
                std.debug.print("--- {s} --- \n{s}\n--- ---\n", .{ match_entry.key_ptr.*, match_entry.value_ptr.* });
            }
        }
    }
}

const RegexError = error { BuildPattern };
const Regex = struct {
    regex: *re.struct_pcre2_real_code_8,

    fn new(needle: []const u8) RegexError!Regex {
        const pattern: re.PCRE2_SPTR8 = &needle[0];
        var errornumber: c_int = undefined;
        var erroroffset: re.PCRE2_SIZE = undefined;
        const regex: ?*re.pcre2_code_8 = re.pcre2_compile_8(
            pattern,
            PCRE2_ZERO_TERMINATED,
            0,
            &errornumber,
            &erroroffset,
            null);

        if (regex == null){
            var errormessage : [256]u8 = undefined;
            const msgLen : c_int = re.pcre2_get_error_message_8(errornumber, &errormessage, errormessage.len);
            std.debug.print("Error compiling: {s}\n",.{errormessage[0..@intCast(msgLen)]});
            return RegexError.BuildPattern;
        }

        return Regex {
            .regex = regex.?,
        };
    }

    fn matches(self: *const Regex, gpa: std.mem.Allocator, haystack: []const u8) !std.StringHashMap([]const u8) {
        const subject: re.PCRE2_SPTR8 = &haystack[0];
        const subjLen: re.PCRE2_SIZE  = haystack.len;

        const matchData: ?*re.pcre2_match_data_8 = re.pcre2_match_data_create_from_pattern_8(self.regex, null);
        const rc: c_int = re.pcre2_match_8(
            self.regex,
            subject,
            subjLen,
            0,
            0,
            matchData.?,
            null
        );

        var final_matches = std.StringHashMap([]const u8).init(gpa);

        if (rc < 0){
            return final_matches;
        }

        const ovector = re.pcre2_get_ovector_pointer_8(matchData);
        if (rc == 0){
            std.debug.print("ovector was not big enough for all the captured substrings\n",.{});
            return final_matches;
        }

        if(ovector[0] > ovector[1]){
            std.debug.print("error with ovector\n",.{});
            re.pcre2_match_data_free_8(matchData);
            re.pcre2_code_free_8(self.regex);
            return final_matches;
        }

        var name_table: ?re.PCRE2_SPTR8 = undefined;
        var name_entry_size: ?c_int = undefined;
        var name_count: ?c_int = undefined;

        _ = re.pcre2_pattern_info_8(
            self.regex,
            re.PCRE2_INFO_NAMETABLE,
            &name_table,
        );
        _ = re.pcre2_pattern_info_8(
            self.regex,
            re.PCRE2_INFO_NAMEENTRYSIZE,
            &name_entry_size,
        );
        _ = re.pcre2_pattern_info_8(
            self.regex,
            re.PCRE2_INFO_NAMECOUNT,
            &name_count,
        );

        var tabptr: re.PCRE2_SPTR8 = name_table.?;
        for (0..@as(usize, @intCast(name_count.?))) |_| {
            const n = (@as(usize, tabptr[0]) << 8) | tabptr[1];
            const match = haystack[ovector[2*n]..ovector[2*n+1]];
            const key = tabptr[2..@as(usize, @intCast(name_entry_size.?))];

            std.debug.print("{d}: {s}\n", .{ n, key });
            try final_matches.put(key, match);

            tabptr += @as(usize, @intCast(name_entry_size.?));
        }

        return final_matches;
    }
};
