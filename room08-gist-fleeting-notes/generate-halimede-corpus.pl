#!/usr/bin/env perl
# https://www.w3.org/TR/webmention/#sending-webmentions
use strict;
use warnings;
use Mojo::DOM;

my @links = (
    "https://halimedemf.substack.com/p/the-penis-monologues-because-she",
    "https://halimedemf.substack.com/p/a-part-at-the-seams-recognizing-differences",
    "https://halimedemf.substack.com/p/queer-safety-and-visibility",
    "https://halimedemf.substack.com/p/an-actual-defense-of-effeminate-boys",
    "https://halimedemf.substack.com/p/things-i-can-tell-about-you-based",
    "https://halimedemf.substack.com/p/purple-girls",
    "https://halimedemf.substack.com/p/halimedes-guide-to-picking-up-trans",
    "https://halimedemf.substack.com/p/stop-boying-yourself",
    "https://halimedemf.substack.com/p/how-to-write-trans-women",
    "https://halimedemf.substack.com/p/why-im-not-nonbinary",
    "https://halimedemf.substack.com/p/trans-women-are-hot-because-of-gendered",
    "https://halimedemf.substack.com/p/bloodborne-feelings",
    "https://halimedemf.substack.com/p/the-gay-screwtape-letters-convincing",
    "https://halimedemf.substack.com/p/defamiliarizing-trans-women",
    "https://halimedemf.substack.com/p/halimedes-list-of-gifts-for-transgender",
    "https://halimedemf.substack.com/p/reasons-you-think-i-am-transgender",
    "https://halimedemf.substack.com/p/a-gun-that-makes-people-stop-being",
    "https://halimedemf.substack.com/p/kinds-of-non-trans-likely-to-be-nice",
    "https://halimedemf.substack.com/p/weight-of-weeaboo-womanhood-i-fujoshi",
    "https://halimedemf.substack.com/p/weight-of-weeaboo-womanhood-ii-himedanshi",
    "https://halimedemf.substack.com/p/how-to-write-a-cis-woman",
    "https://halimedemf.substack.com/p/trans-exclusionary-radical-fanatics",
    "https://halimedemf.substack.com/p/against-gender-identity",
    "https://halimedemf.substack.com/p/fanfiction-i-am-never-going-to-write-6c9",
    "https://halimedemf.substack.com/p/why-does-she-do-that",
    "https://halimedemf.substack.com/p/the-tokyo-yushun-is-decadent-and",
    "https://halimedemf.substack.com/p/what-are-halimede-impersonators-really",
    "https://halimedemf.substack.com/p/fanfiction-i-am-never-going-to-write",
    "https://halimedemf.substack.com/p/a-cis-girls-guide-to-picking-up-trans"
);

my $corpus = "";
my $title_corpus = "";
for my $post_url (@links[0..0]) {
    print "[DEBUG]: Fetching $post_url\n";
    my $post = `curl -s -X GET '$post_url'`;
    my $dom = Mojo::DOM->new;
    print "[DEBUG]: Parsing $post_url\n";
    $dom->parse($post);

    my $post_title = $dom->at('.post-title')->all_text;
    my $subtitle = $dom->at('.subtitle');
    if (defined $subtitle) {
        $subtitle = "\n" . $subtitle->all_text;
    } else {
        $subtitle = "";
    }

    my $content = "";
    # Find text thats not tweet or quote of tweet and not widget and not footnotes
    for my $el ($dom->find('.available-content > .body > *:not(.subscription-widget-wrap):not(.footnote):not(blockquote):not(.pencraft)')->each) {
        my $el_content = "";
        # Remove footnotes anchors
        for my $anchor ($el->find('.footnote-anchor')->each) {
            $anchor->remove();
        }
        if ($el->tag eq "ul") {
            $el_content = $el->children('li')->map(sub { "- " . $_->all_text })->join("\n");
        } elsif ($el->tag eq "ol") {
            my $start_at = 1;
            my $start_attr = $el->attr('start');
            if ($start_attr) {
                $start_at = $start_attr;
            }
            $el_content = $el->children('li')->map(sub { $_->preceding_nodes->size + $start_at . ". " . $_->all_text })->join("\n");
        } else {
            $el_content = $el->all_text;
        }

        $content = $content . $el_content . "\n";
    }
    $content =~ s/(^\s*\n|^\s*)//gm;
    $content = $post_title
        . $subtitle
        . "\n\n"
        . $content;

    $title_corpus = $title_corpus . $post_title . "\n";
    $corpus = $corpus . $content . "\n\n\n";
}

my $corpus_file = "halimede-corpus.txt";
my $fh_corpus;
open( $fh_corpus, ">", $corpus_file ) || die "Can't open $corpus_file: $!";
print $fh_corpus $corpus;
close $fh_corpus;

my $title_corpus_file = "halimede-title-corpus.txt";
my $fh_title_corpus;
open( $fh_title_corpus, ">", $title_corpus_file ) || die "Can't open $corpus_file: $!";
print $fh_title_corpus $title_corpus;
close $fh_title_corpus;

# List a.external links
# print "[INFO]: Extracting external links\n";
# my %file_with_links;
# my %file_hashes;
# for my $file (@files) {
#     open( my $fh, "<", $file ) || die "Can't open $file: $!";
#     my $html = join("", <$fh>); # Read entire file content
#     close $fh;

#     # Search file for external links
#     my $dom = Mojo::DOM->new;
#     $dom->parse($html);
#     for my $el ($dom->find('#main-content a[rel*="external"]')->each) {
#         my $href = $el->{href};
#         # Store link if not already in array
#         push @{$file_with_links{$file}}, $href unless grep { $_ eq $href } @{$file_with_links{$file}};
#     }

#     # Calculate SHA256 hash of the content only
#     use Digest::SHA;
#     # Use page content
#     my $content = $dom->at('#main-content .e-content');
#     # Or use page heading
#     if (!$content) { $content = $dom->at('#main-content p:not(:has(small))') }

#     if ($content) {
#         $content = $content->all_text;
#         $content =~ s/(^\s*\n|^\s*)//gm;
#         $file_hashes{$file} = Digest::SHA->new(256)->add($content)->hexdigest;
#     } else {
#         # Or by default use file name (so no update will be sent)
#         $file_hashes{$file} = Digest::SHA->new(256)->add($file)->hexdigest;
#     }
# }
# my @unique_links;
# for my $file (keys %file_with_links) {
#     for my $link (@{$file_with_links{$file}}) {
#         push @unique_links, $link unless grep { $_ eq $link } @unique_links;
#     }
# }
# print "[INFO]: Extracted " . scalar @unique_links . " unique links\n";

# # Function to extract the webmention URL from a given URL
# sub get_webmention_endpoint {
#     my ($url) = @_;
#     my $html = `curl -sL "$url"`;
#     my $dom = Mojo::DOM->new;
#     $dom->parse($html);

#     # Look for <link rel="webmention" href="...">
#     for my $el ($dom->find('link[rel="webmention"]')->each) {
#         return $el->{href} if $el->{href};
#     }

#     # If not found, look for <link rel="webmention" href="..."> with a space-separated rel value
#     for my $el ($dom->find('link[rel*="webmention"]')->each) {
#         my @rels = split /\s+/, $el->{rel};
#         for my $rel_val (@rels) {
#             return $el->{href} if $rel_val eq 'webmention' && $el->{href};
#         }
#     }

#     return undef; # Return undef if no webmention endpoint is found
# }

# # Restoring state
# my $state_file = "scripts/webmentions.csv";

# my %webmention_endpoints;
# my %previous_files_with_links;
# my %previous_files_hash;
# if (-e $state_file) {
#     open(my $fh_read, "<", $state_file) || die "Can't open $state_file: $!";
#     while (my $line = <$fh_read>) {
#         chomp $line;
#         my @parts = split /,/, $line, 4; # html_file, url, webmention_endpoint, hash
#         if (scalar @parts == 4) {
#             my ($html_file, $url, $webmention_endpoint, $hash) = @parts;
#             $webmention_endpoints{$url} = ($webmention_endpoint eq "") ? undef : $webmention_endpoint;
#             push @{$previous_files_with_links{$html_file}}, $url;
#             $previous_files_hash{$html_file} = $hash;
#         }
#     }
#     close $fh_read;
#     print "[INFO]: Restored " . scalar(keys %webmention_endpoints) . " webmention endpoints from $state_file\n";
# }

# # For each unique link, try to find its webmention endpoint
# for my $url (@unique_links) {
#     # Try the full URL first
#     if (exists $webmention_endpoints{$url}) {
#         # Usefull to debug
#         # print "Webmention endpoint for $url already in cache: " . (defined $webmention_endpoints{$url} ? $webmention_endpoints{$url} : "None") . "\n";
#     } else {
#         my $endpoint = get_webmention_endpoint($url);
#         if (defined $endpoint) {
#             $webmention_endpoints{$url} = $endpoint;
#             print "Found webmention endpoint for $url: $endpoint\n";
#         } else {
#             $webmention_endpoints{$url} = undef;
#             print "No webmention endpoint found for $url\n";
#         }
#     }

#     # TOREVIEW: Is it really usefull to check the root domain like it could make mentions to the wrong target
#     # If no endpoint was found for the full URL, or if we want to check the root anyway,
#     # try the root URL (without path)
#     # my $root_url;
#     # if ($url =~ m|^(https?://[^/]+/)|) {
#     #     $root_url = $1;
#     # }

#     # if (defined $root_url && $root_url ne $url && !defined $webmention_endpoints{$url}) {
#     #     if (exists $webmention_endpoints{$root_url}) {
#     #         # Usefull to debug
#     #         # print "Webmention endpoint for root $root_url already in cache: " . (defined $webmention_endpoints{$root_url} ? $webmention_endpoints{$root_url} : "None") . "\n";
#     #     } else {
#     #         my $endpoint = get_webmention_endpoint($root_url);
#     #         if (defined $endpoint) {
#     #             $webmention_endpoints{$root_url} = $endpoint;
#     #             $webmention_endpoints{$url} = $endpoint;
#     #             print "Found webmention endpoint for root $root_url: $endpoint\n";
#     #         } else {
#     #             $webmention_endpoints{$root_url} = undef;
#     #             print "No webmention endpoint found for root $root_url\n";
#     #         }
#     #     }
#     # }
# }

# my @defined_endpoints = grep { defined $webmention_endpoints{$_} } keys %webmention_endpoints;
# print "[INFO]: Collected " . scalar(@defined_endpoints) . " webmention endpoints.\n";

# # Storing state in $state_file html_file, url, webmention_endpoint
# my $fh_state;
# if (! $dry_run) {
#     open( $fh_state, ">", $state_file ) || die "Can't open $state_file: $!";
# }
# for my $file (keys %file_with_links) {
#     for my $url (@{$file_with_links{$file}}) {
#         my $endpoint_value = defined $webmention_endpoints{$url} ? $webmention_endpoints{$url} : "";
#         if (! $dry_run) {
#             print $fh_state "$file,$url,$endpoint_value,$file_hashes{$file}\n";
#         }
#     }
# }
# if (! $dry_run) {
#     close $fh_state;
# }
# print "[INFO]: Stored webmention data in $state_file\n";

# # https://stackoverflow.com/questions/18103501/prompting-multiple-questions-to-user-yes-no-file-name-input
# sub prompt {
#     my ($query) = @_; # take a prompt string as argument
#     local $| = 1; # activate autoflush to immediately show the prompt
#     print $query;
#     chomp(my $answer = <STDIN>);
#     return $answer;
# }

# sub prompt_yn {
#     my ($query) = @_;
#     my $answer = prompt("$query (Y/N): ");
#     return lc($answer) eq 'y';
# }

# # Function to send webmention
# sub send_webmention {
#     my ($endpoint, $source_url, $target_url) = @_;

#     # If the endpoint starts with '/', prepend the root of the target_url
#     if ($endpoint =~ m#^/#) {
#         if ($target_url =~ m#^(https?://[^/]+)#) {
#             my $target_root = $1;
#             $endpoint = $target_root . $endpoint;
#         }
#     }

#     print "[DEBUG]: Sending source=$source_url target=$target_url via=$endpoint\n";
#     my $cmd = "curl -s -o /dev/null -w '%{http_code}' " .
#               "--data-urlencode 'source=$source_url' " .
#               "--data-urlencode 'target=$target_url' " .
#               "-H \"Content-Type: application/x-www-form-urlencoded\" " .
#               "-X POST '$endpoint'";
#     my $send_req = prompt_yn("Do you want to send a webmention to $target_url (endpoint: $endpoint)");
#     my $http_code;
#     if ((! $dry_run) && $send_req) {
#         $http_code = `$cmd`;
#     }
#     if (!defined $http_code) {
#         print "[DEBUG]: Webmention not sent \n";
#         return 0;
#     } elsif ($http_code =~ /^(200|201|202)$/) {
#         print "[DEBUG]: Webmention sent successfully (HTTP $http_code)\n";
#         return 1;
#     } else {
#         print "[DEBUG]: Failed to send webmention (HTTP $http_code)\n";
#         return 0;
#     }
# }

# # Construct source_url from file
# sub source_url_from_file {
#     my ($file) = @_;
#     my $source_path = $file;
#     $source_path =~ s/^public\///; # Strip 'public/' prefix
#     $source_path =~ s/\.html$//;   # Replace '.html' with empty string
#     $source_path =~ s/\/index$//;  # Remove '/index' if it's the end of the path
#     return $base_url . $source_path . "/"; # Append base_url and add a trailing slash
# }

# # Handle deleted files (to send a final update)
# print "[INFO]: Handling deleted files\n";
# for my $prev_file (keys %previous_files_with_links) {
#     if (!exists $file_with_links{$prev_file}) {
#         print "[INFO]: --- File $prev_file was deleted. Attempting to send webmentions for its links. ---\n";
#         for my $url (@{$previous_files_with_links{$prev_file}}) {
#             if (defined $webmention_endpoints{$url}) {
#                 send_webmention($webmention_endpoints{$url}, source_url_from_file($prev_file), $url);
#             }
#         }
#     }
# }

# # Send new/updated webmentions
# print "[INFO]: Handling new and updated files\n";
# for my $file (keys %file_with_links) {
#     my $source_url = source_url_from_file($file);

#     # Check if the file is new or has been modified
#     if (!exists $previous_files_hash{$file} || $previous_files_hash{$file} ne $file_hashes{$file}) {
#         my $previous_hash = "";
#         if (defined $previous_files_hash{$file}) {
#             $previous_hash = $previous_files_hash{$file};
#         }
#         print "[INFO]: --- Content of $file is new or modified. Sending webmentions. ---\n";
#         print "[DEBUG]: Was    => $previous_hash\n";
#         print "[DEBUG]: Is now => $file_hashes{$file}\n";
#         for my $url (@{$file_with_links{$file}}) {
#             if (defined $webmention_endpoints{$url}) {
#                 send_webmention($webmention_endpoints{$url}, $source_url, $url);
#             }
#         }
#     }
# }
