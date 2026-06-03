const std = @import("std");


pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe_mod = b.addModule("illiterate", .{
        .root_source_file = b.path("zig/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // === START deps ===
    const pcre2_dep = b.dependency("pcre2", .{
        .target = target,
        .optimize = optimize,
    });
    exe_mod.linkLibrary(pcre2_dep.artifact("pcre2-8"));
    // === END deps ===

    // === START default target ===
    const exe = b.addExecutable(.{
        .name = "illiterate",
        .root_module = exe_mod,
    });
    b.installArtifact(exe);
    // === END default target ===

    // === START check target ===
    const exe_check = b.addExecutable(.{
        .name = "illiterate",
        .root_module = exe_mod,
    });
    const check = b.step("check", "Check if foo compiles");
    check.dependOn(&exe_check.step);
    // === END check target ===

    // === START run target ===
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
    // === END run target ===
}
