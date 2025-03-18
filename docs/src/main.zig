const std = @import("std");
// const []const  std[]const .
const ArrayList = std.ArrayList;
const json = @import("./json_defines.zig");
const utils = @import("./utils.zig");
const base_source_url = "https://github.com/vnuxa/astrum/blob/unstable/src/lua_library/astrum/types/";

//
// NOTE: maybe add start finish too to this


pub const DefinitionObject = struct {
    path: []const u8,
    name: []const u8
};

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr(`
    // std.debug.print("All your {s} are belong to us.\n", .{"codebase"});



    var gpa = std.heap.GeneralPurposeAllocator(.{ .verbose_log = false }){};
    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "./src/doc_out/doc.json", 9216000);
    defer allocator.free(data);

    // clear the directory first
    _ = try std.fs.cwd().deleteTree("./main/auto_gen/");

    const json_object: std.json.Parsed([]json.DocObject) = try std.json.parseFromSlice([]json.DocObject, allocator, data, .{
        .ignore_unknown_fields = true
    });
    defer json_object.deinit();

    var content_tables_index = std.StringHashMap(u8).init(allocator);
    var definition_objects = std.StringHashMap(DefinitionObject).init(allocator);
    // var objects = std.StringHashMap(json.DocObject).init(allocator);

    for (json_object.value) |object| {
        if (object.defines[0].file) |file| {
            if (file.len > 9 and (std.mem.eql(u8,file[0..9], "[FOREIGN]") or std.mem.eql(u8,file[0..9], "[FORIEGN]"))) { continue; }

            const mdfile_name = try std.fmt.allocPrint(allocator, "{s}{s}{s}",  .{"./main/auto_gen/", file[0..(file.len-3)], "md"});

            const mdfile_dir = utils.get_parent_dir(mdfile_name);
            // gets the directory for the file ( removes text after final / )

            try std.fs.cwd().makePath(mdfile_dir);

            var mdfile: std.fs.File = undefined;
            if (std.fs.cwd().openFile(mdfile_name, .{ .mode = .write_only})) |md_file| {
                mdfile = md_file;
            } else |_| {
                mdfile = try std.fs.cwd().createFile(mdfile_name, .{});
                _= try mdfile.write("# Table of contents\n\n");
                try content_tables_index.put(file, 1);
            }

            try mdfile.seekFromEnd(0);

            // add definition to table of contents
            {
                const index = content_tables_index.get(file) orelse unreachable;
                var buf: [40]u8 = undefined;
                try utils.write_format(mdfile, allocator, "{o}. [`{s}`](#{s}) \n", .{ index, object.defines[0].view, std.ascii.lowerString(&buf, object.defines[0].view) });
                try content_tables_index.put(file, index + 1);
            }

            const object_name = object.defines[0].view;
            try definition_objects.put(object_name, DefinitionObject { .name =  object_name, .path = file[0..(file.len-4)] });

            mdfile.close();

        }
    }

    // add to summary sidebar thing

    try std.fs.cwd().deleteFile("./main/SUMMARY.md");
    var summary: std.fs.File = try std.fs.cwd().createFile("./main/SUMMARY.md", .{});

    _ = try summary.write(try (try std.fs.cwd().openFile("./main/manual_summary.md", .{})).readToEndAlloc(allocator, 9216000));
    _ = try summary.write("\n---\n");
    _ = try summary.write("# Types\n");

    const main_file: std.fs.Dir = try std.fs.cwd().openDir("./main/auto_gen", std.fs.Dir.OpenDirOptions { .iterate = true });
    var main_file_iterator = main_file.iterate();

    _ = try summary.write("- [Astrum](./auto_gen/init.md)\n");
    while (try main_file_iterator.next()) |value| {
        const value_thing: std.fs.Dir.Entry = value;
        if (value.kind == std.fs.File.Kind.directory) {
            try utils.write_format(
                summary,
                allocator,
                "   - [{c}{s}](./auto_gen/{s}/init.md)\n",
                .{ std.ascii.toUpper(value_thing.name[0]), value_thing.name[1..], value_thing.name }
            );
            var dir_iterator: std.fs.Dir.Iterator = (
                try std.fs.cwd().openDir(
                    try std.fmt.allocPrint(allocator, "./main/auto_gen/{s}", .{ value_thing.name }),
                    std.fs.Dir.OpenDirOptions { .iterate = true }
                )
            ).iterate();

            while (try dir_iterator.next()) |file| {
                if (file.kind == std.fs.File.Kind.file and !std.mem.eql(u8, file.name, "init.md")) {
                    try utils.write_format(
                        summary,
                        allocator,
                        "       - [{c}{s}](./auto_gen/{s}/{s})\n",
                        .{ std.ascii.toUpper(file.name[0]), file.name[1.. file.name.len-3], value_thing.name, file.name }
                    );
                }
            }
        }
        std.debug.print("HEADER: a value thing is {s}\n", .{value_thing.name});
    }

    try utils.write_format(summary, allocator, "\n[Last auto generated: {s}]()\n\n---\n", .{ utils.current_date(allocator) catch "" });


    summary.close();


    // write the actual md pages in those directories

    var has_first_entry = std.StringHashMap(bool).init(allocator);
    for (json_object.value) |object| {
        if (object.defines[0].file) |file| {
            // std.debug.print("name: {s}\n", .{file});
            if (file.len > 9 and (std.mem.eql(u8,file[0..9], "[FOREIGN]") or std.mem.eql(u8,file[0..9], "[FORIEGN]"))) { continue; }
            const mdfile_name = try std.fmt.allocPrint(allocator, "{s}{s}{s}",  .{"./main/auto_gen/", file[0..(file.len-3)], "md"});
            var mdfile = try std.fs.cwd().openFile(mdfile_name, .{ .mode = .write_only});
            defer mdfile.close();

            try mdfile.seekFromEnd(0);
            if (has_first_entry.get(file) orelse false != true) {
                try utils.write_format(mdfile, allocator, "\n[`source`]({s}{s})\n", .{ base_source_url, file });
                _ = try mdfile.write("\n---\n");
                try has_first_entry.put(file, true);
            }

            const object_name = object.defines[0].view;

            try utils.write_format(mdfile, allocator, "# {s}\n## Propreties:\n", .{object_name});
            var methods = ArrayList(json.DocField).init(allocator);
            var propreties = ArrayList(json.DocField).init(allocator);

            for (object.fields orelse unreachable) |field| {
                if (std.mem.eql(u8,field.type, "setmethod")) {
                    try methods.append(field);
                } else if (std.mem.eql(u8, field.type, "doc.field")) {
                    std.debug.print("PROPRETY: {s} FOR OBJECT: {s}\n", .{ field.name, object_name });
                    try propreties.append(field);
                }
            }
            const mdfile_dir = utils.get_parent_dir(mdfile_name);

            for (propreties.items) |proprety| {
                try utils.write_format(mdfile, allocator, ">   `{s}` → `{s}`\n", .{ proprety.name, proprety.view});
                if (proprety.desc) |description| {
                    try utils.write_format(mdfile, allocator, ">    >   {s} \n", .{ description });
                }
                // IMPORTANT: add a see definition!
                // see if the text contains words of the thing
                const more_definitions: []const u8 = try utils.get_definitions(allocator, definition_objects, proprety, mdfile_name, mdfile_dir);

                _ = try mdfile.write("\n");

                if ( more_definitions.len != 17) {
                    _ = try mdfile.write(more_definitions);
                    _ = try mdfile.write("\n");
                }
            }

            _ = try mdfile.write("## Methods:\n");
            for (methods.items) |method| {
                var arguments: []const u8  = "";
                if (method.extends.args) |extends| {
                    var is_first = true;
                    for (extends) |value| {
                        if (std.mem.eql(u8, value.name orelse unreachable, "self")) { continue; }
                        if (is_first) {
                            arguments = value.name orelse unreachable;
                            is_first = false;
                        } else {
                            std.debug.print("wow uh we are changign arguments\n", .{});
                            arguments = try std.fmt.allocPrint(allocator, "{s}, {s}", .{ arguments, value.name orelse unreachable });
                        }
                    }
                }


                var returns_list: ?[]const u8  = null;
                if (method.extends.returns) |extends| {
                    var is_first = true;
                    for (extends) |value| {
                        if (is_first) {
                            returns_list = value.view;
                            is_first = false;
                        } else {
                            returns_list = try std.fmt.allocPrint(allocator, "{s}, {s}", .{ returns_list orelse unreachable, value.view });
                        }
                   }
                }

                if (returns_list) |returns| {
                    try utils.write_format(mdfile, allocator, "`:{s}({s})` → `{s}`\n", .{ method.name, arguments, returns });
                } else {
                    try utils.write_format(mdfile, allocator, "`:{s}({s})`\n", .{ method.name, arguments});
                }

                if (method.extends.args) |extends| {
                    for (extends) |value| {
                        if (std.mem.eql(u8, value.name orelse unreachable, "self")) { continue; }
                        try utils.write_format(mdfile, allocator, ">    `{s}`: `{s}`\n", .{ value.name orelse unreachable, value.view });
                        if (value.desc) |description| {
                            try utils.write_format(mdfile, allocator, ">    >   {s} \n", .{ description });
                        }
                        _ = try mdfile.write("\n");
                    }
                }
                _ = try mdfile.write("\n");
                const more_definitions: []const u8 = try utils.get_definitions(allocator, definition_objects, method, mdfile_name, mdfile_dir);

                if ( more_definitions.len != 17) {
                    _ = try mdfile.write("\n");
                    _ = try mdfile.write(more_definitions);
                    _ = try mdfile.write("\n");
                }
                // code to do methods
                // _ = try mdfile.write(try std.fmt.allocPrint(allocator, ">   `{s}` → `{s}`\n>    >   {s}", .{ proprety.name, proprety.view, proprety.desc }));

            }
            _ = try mdfile.write("\n\n---\n");
        }

    }

}
