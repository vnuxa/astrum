const std = @import("std");
// const []const  std[]const .
const ArrayList = std.ArrayList;
const json = @import("./json_defines.zig");
const utils = @import("./utils.zig");

//
// NOTE: maybe add start finish too to this


pub const DefinitionObject = struct {
    path: []const u8,
    name: []const u8
};

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr(`
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});



    var gpa = std.heap.GeneralPurposeAllocator(.{ .verbose_log = false }){};
    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "./src/doc.json", 9216000);
    defer allocator.free(data);

    // clear the directory first
    _ = try std.fs.cwd().deleteTree("./docs/auto_gen/");

    const json_object: std.json.Parsed([]json.DocObject) = try std.json.parseFromSlice([]json.DocObject, allocator, data, .{
        .ignore_unknown_fields = true
    });
    defer json_object.deinit();

    var content_tables_index = std.StringHashMap(u8).init(allocator);
    var definition_objects = std.StringHashMap(DefinitionObject).init(allocator);
    // var objects = std.StringHashMap(json.DocObject).init(allocator);

    for (json_object.value) |object| {
        if (object.defines[0].file) |file| {
            if (std.mem.eql(u8,file[0..9], "[FOREIGN]")) { continue; }

            const mdfile_name = try std.fmt.allocPrint(allocator, "{s}{s}{s}",  .{"./docs/auto_gen/", file[0..(file.len-3)], "md"});

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
                try utils.write_format(mdfile, allocator, "{o}. [`{s}`](#{s}) \n", .{ index, object.defines[0].view,object.defines[0].view });
                try content_tables_index.put(file, index + 1);
            }

            const object_name = object.defines[0].view;
            try definition_objects.put(object_name, DefinitionObject { .name =  object_name, .path = file[0..(file.len-3)] });

            mdfile.close();
        }
    }

    var has_first_entry = std.StringHashMap(bool).init(allocator);
    for (json_object.value) |object| {
        if (object.defines[0].file) |file| {
            if (std.mem.eql(u8,file[0..9], "[FOREIGN]")) { continue; }
            const mdfile_name = try std.fmt.allocPrint(allocator, "{s}{s}{s}",  .{"./docs/auto_gen", file[0..(file.len-3)], "md"});
            var mdfile = try std.fs.cwd().openFile(mdfile_name, .{ .mode = .write_only});
            defer mdfile.close();

            try mdfile.seekFromEnd(0);
            if (has_first_entry.get(file) orelse false != true) {
                _ = try mdfile.write("\n---\n");
                try has_first_entry.put(file, true);
            }

            const object_name = object.defines[0].view;

            try utils.write_format(mdfile, allocator, "# {s}\n## Propreties:\n", .{object_name});
            var methods = ArrayList(json.DocField).init(allocator);
            var propreties = ArrayList(json.DocField).init(allocator);

            for (object.fields) |field| {
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
                    try utils.write_format(mdfile, allocator, ">    >   {s}\n", .{ description });
                }
                // IMPORTANT: add a see definition!
                // see if the text contains words of the thing
                const more_definitions: []const u8 = try utils.get_definitions(allocator, definition_objects, proprety, object_name, mdfile_name, mdfile_dir);


                if ( more_definitions.len != 17) {
                    _ = try mdfile.write("\n");
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
                        if (std.mem.eql(u8, value.name, "self")) { continue; }
                        if (is_first) {
                            arguments = value.name;
                            is_first = false;
                        } else {
                            std.debug.print("wow uh we are changign arguments\n", .{});
                            arguments = try std.fmt.allocPrint(allocator, "{s}, {s}", .{ arguments, value.name });
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
                        if (std.mem.eql(u8, value.name, "self")) { continue; }
                        try utils.write_format(mdfile, allocator, ">    `{s}`: `{s}`\n", .{ value.name, value.view });
                        if (value.desc) |description| {
                            try utils.write_format(mdfile, allocator, ">    >   {s}\n", .{ description });
                        }
                        _ = try mdfile.write("\n");
                    }
                }
                _ = try mdfile.write("\n");
                const more_definitions: []const u8 = try utils.get_definitions(allocator, definition_objects, method, object_name, mdfile_name, mdfile_dir);

                if ( more_definitions.len != 17) {
                    _ = try mdfile.write("\n");
                    _ = try mdfile.write(more_definitions);
                    _ = try mdfile.write("\n");
                }
                // code to do methods
                // _ = try mdfile.write(try std.fmt.allocPrint(allocator, ">   `{s}` → `{s}`\n>    >   {s}", .{ proprety.name, proprety.view, proprety.desc }));

            }
            _ = try mdfile.write("\n---\n");
        }

    }



    //


    // for (file_definitions.ctx) |value| {}
    // while (value_iterator.next()) |entry| {
    //     const file = entry.defines[0].file orelse unreachable;
    //     const mdfile_name = try std.fmt.allocPrint(allocator, "{s}{s}{s}",  .{"./docs", file[0..(file.len-3)], "md"});
    //     var mdfile = try std.fs.cwd().openFile(mdfile_name, .{ .mode = .write_only});
    //     defer mdfile.close();
    //
    //     try mdfile.seekFromEnd(0);
    //     if (has_first_entry.get(file) orelse false != true) {
    //         _ = try mdfile.write("\n---\n");
    //         try has_first_entry.put(file, true);
    //     }
    //
    //         try utils.write_format(mdfile, allocator, "# {s}\n## Propreties:\n", .{entry.defines[0].view});
    //         var methods = ArrayList(json.DocField).init(allocator);
    //         var propreties = ArrayList(json.DocField).init(allocator);
    //
    //         for (entry.fields) |field| {
    //             if (std.mem.eql(u8,field.type, "setmethod")) {
    //                 try methods.append(field);
    //             } else if (std.mem.eql(u8, field.type, "doc.field")) {
    //                 std.debug.print("PROPRETY: {s} FOR OBJECT: {s}\n", .{ field.name, entry.defines[0].view });
    //                 try propreties.append(field);
    //             }
    //         }
    //
    //         for (propreties.items) |proprety| {
    //             try utils.write_format(mdfile, allocator, ">   `{s}` → `{s}`\n", .{ proprety.name, proprety.view});
    //             if (proprety.desc) |description| {
    //                 try utils.write_format(mdfile, allocator, ">    >   {s}\n", .{ description });
    //             }
    //         // IMPORTANT: add a see definition!
    //         }
    //
    //
    // }

    // for (json_object.value) |object| {
    //     if (object.defines[0].file) |file| {
    //         // TODO: make a function that makes the directoryu of file if it dont exist
    //         if (std.mem.eql(u8,file[0..9], "[FOREIGN]")) { continue; }
    //         const mdfile_name = try std.fmt.allocPrint(allocator, "{s}{s}{s}",  .{"./docs", file[0..(file.len-3)], "md"});
    //         var mdfile_dir = mdfile_name;
    //
    //         {
    //             var i: usize = mdfile_dir.len;
    //
    //             while (i > 0) : (i -= 1) {
    //                 if (mdfile_dir[i-1] != '/') {
    //                     mdfile_dir = mdfile_dir[0..i-1];
    //                 } else {
    //                     break;
    //                 }
    //             }
    //         }
    //
    //         try std.fs.cwd().makePath(mdfile_dir);
    //         var mdfile: std.fs.File = undefined;
    //         if (std.fs.cwd().openFile(mdfile_name, .{ .mode = .write_only})) |md_file| {
    //             std.debug.print("file already found!: {s}\n", .{mdfile_name});
    //             mdfile = md_file;
    //         } else |_| {
    //             mdfile = try std.fs.cwd().createFile(mdfile_name, .{});
    //             _= try mdfile.write("# Table of contents \n\nhello");
    //             try content_tables_offset.put(file, 23);
    //             try content_tables_index.put(file, 1);
    //             // _ = try mdfile.write("\n---TESTINGGGGGGGGGG\n");
    //         }
    //
    //         // {
    //         //     const old_offset = content_tables_offset.get(file) orelse unreachable;
    //         //     const index = content_tables_index.get(file) orelse unreachable;
    //         //     try mdfile.seekTo(old_offset - 1);
    //         //     // _ = try mdfile.write("hiii");
    //         //     const line_offset: u64 = @intCast(
    //         //         try utils.write_format_returns(mdfile, allocator, "{o}. [`{s}`](#{s}) \n", .{ index, object.defines[0].view,object.defines[0].view  })
    //         //     );
    //         //
    //         //     try content_tables_offset.put(file, old_offset + line_offset);
    //         //     try content_tables_index.put(file, index + 1);
    //         // }
    //         //
    //         //
    //         // try mdfile.seekFromEnd(0);
    //
    //
    //         // _ = try mdfile.write(try std.fmt.allocPrint(allocator, "#{s}\n##Propreties:\n", .{object.defines[0].view}));
    //         try utils.write_format(mdfile, allocator, "# {s}\n## Propreties:\n", .{object.defines[0].view});
    //         var methods = ArrayList(json.DocField).init(allocator);
    //         var propreties = ArrayList(json.DocField).init(allocator);
    //
    //         for (object.fields) |field| {
    //             if (std.mem.eql(u8,field.type, "setmethod")) {
    //                 try methods.append(field);
    //             } else if (std.mem.eql(u8, field.type, "doc.field")) {
    //                 std.debug.print("PROPRETY: {s} FOR OBJECT: {s}\n", .{ field.name, object.defines[0].view });
    //                 try propreties.append(field);
    //             }
    //         }
    //
    //         for (propreties.items) |proprety| {
    //             // NOTE: this is for the lsp..
    //             // var proprety: DocField = proprety;
    //             try utils.write_format(mdfile, allocator, ">   `{s}` → `{s}`\n", .{ proprety.name, proprety.view});
    //             if (proprety.desc) |description| {
    //                 try utils.write_format(mdfile, allocator, ">    >   {s}\n", .{ description });
    //
    //             }
    //             // TODO: add a see defintiion thing
    //         }
    //
    //         _ = try mdfile.write("## Methods:\n");
    //         for (methods.items) |method| {
    //             var arguments: []const u8  = "";
    //             if (method.extends.args) |extends| {
    //                 var is_first = true;
    //                 for (extends) |value| {
    //                     if (std.mem.eql(u8, value.name, "self")) { continue; }
    //                     if (is_first) {
    //                         arguments = value.name;
    //                         is_first = false;
    //                     } else {
    //                         std.debug.print("wow uh we are changign arguments\n", .{});
    //                         arguments = try std.fmt.allocPrint(allocator, "{s}, {s}", .{ arguments, value.name });
    //                     }
    //                 }
    //             }
    //
    //
    //             var returns_list: ?[]const u8  = null;
    //             if (method.extends.returns) |extends| {
    //                 var is_first = true;
    //                 for (extends) |value| {
    //                     if (is_first) {
    //                         returns_list = value.view;
    //                         is_first = false;
    //                     } else {
    //                         // std.debug.print("wow uh we are changign arguments ON RETURN thing \n", .{});
    //                         returns_list = try std.fmt.allocPrint(allocator, "{s}, {s}", .{ returns_list orelse unreachable, value.view });
    //                     }
    //                }
    //             }
    //
    //             if (returns_list) |returns| {
    //                 try utils.write_format(mdfile, allocator, "`:{s}({s})` → `{s}`\n", .{ method.name, arguments, returns });
    //             } else {
    //                 try utils.write_format(mdfile, allocator, "`:{s}({s})`\n", .{ method.name, arguments});
    //             }
    //
    //             if (method.extends.args) |extends| {
    //                 for (extends) |value| {
    //                     if (std.mem.eql(u8, value.name, "self")) { continue; }
    //                     try utils.write_format(mdfile, allocator, ">    `{s}`: `{s}`\n", .{ value.name, value.view });
    //                     if (value.desc) |description| {
    //                         try utils.write_format(mdfile, allocator, ">    >   {s}\n", .{ description });
    //                     }
    //                     _ = try mdfile.write("\n");
    //                 }
    //             }
    //             _ = try mdfile.write("\n");
    //
    //             // code to do methods
    //             // _ = try mdfile.write(try std.fmt.allocPrint(allocator, ">   `{s}` → `{s}`\n>    >   {s}", .{ proprety.name, proprety.view, proprety.desc }));
    //
    //         }
    //         _ = try mdfile.write("\n---\n");
    //
    //         mdfile.close();
    //
    //         methods.deinit();
    //     }
    // }



}
