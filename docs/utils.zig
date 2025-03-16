const std = @import("std");
const main = @import("main.zig");
const json = @import("json_defines.zig");


pub fn get_definitions(
    allocator: std.mem.Allocator,
    definition_objects: std.StringHashMap(main.DefinitionObject),
    proprety: json.DocField,
    object_name: []const u8,
    mdfile_name: []const u8,
    mdfile_dir: []const u8,
) ![]const u8 {
    var definition_iterator = definition_objects.iterator();
    var more_definitions: []const u8 = "see definitions: "; // length is 21
    while (definition_iterator.next()) |entry| {
        if (std.mem.containsAtLeast(u8, proprety.view, 1, entry.key_ptr.*)) {
            const value: main.DefinitionObject = entry.value_ptr.*;
            if (std.mem.eql(u8, value.name, object_name)) { std.debug.print("--sasme name?: {s} in proprety: {s}\n", .{object_name, proprety.name});  continue; }
            // if (std.mem.containsAtLeast(u8, value.path, 1, file)) {
            // return "";
            // }
            // std.debug.print("!!!!!!!!!!!!!!!!!!!!! DIR PATH: {s}, LEN IS: {o}\n", .{ mdfile_dir, mdfile_dir.len });
            var relative_path: ?[]const u8 = null;
            if (mdfile_dir.len == 15) {
                relative_path = try std.fmt.allocPrint(allocator, ".{s}md", .{value.path});
            } else if (std.mem.eql(u8, value.path[1..], mdfile_name[16..mdfile_name.len-2])) {
                // std.debug.print("!!!!!!!!!!!!!!!!!!!!!!!!!!????????? SAME FILE: {s} AND {s}\n", .{ value.path, mdfile_dir[7..]});
                relative_path = "";
            } else if (std.mem.containsAtLeast(u8, value.path, 1, mdfile_dir[7..]) ) {
                relative_path = try std.fmt.allocPrint(allocator, "./{s}md", .{ get_file_name(value.path) });
            } else {
                relative_path = try std.fmt.allocPrint(allocator, "..{s}", .{value.path});
            }
            more_definitions = try std.fmt.allocPrint(allocator, "{s}[`{s}`]({s}#{s}) ", .{ more_definitions, value.name, relative_path orelse "", value.name });
        }
    }

    return more_definitions;
}

pub fn write_format(file: std.fs.File,allocator: std.mem.Allocator, comptime format: []const u8, args: anytype) !void {
    const string = try std.fmt.allocPrint(allocator, format, args);
    defer allocator.free(string);

    _ = try file.writeAll(string);
}

pub fn write_format_returns(file: std.fs.File,allocator: std.mem.Allocator, comptime format: []const u8, args: anytype) !usize {
    const string = try std.fmt.allocPrint(allocator, format, args);
    defer allocator.free(string);

    return try file.write(string);
}


pub fn combine_relative_path(source: []const u8, destination: []const u8) []const u8 {
    if (source.len == 6) {
    }
    if (std.mem.containsAtLeast(u8, destination, 1, source)) {
        return "";
    }
    var i: usize = 1;
    std.debug.print("HELLO I GOT DEST INATION: {s}\n", .{ destination });
    std.debug.print("HELLO I GOT SOURCE : {s}\n", .{ source });
    while (i < source.len and i < destination.len)  {
        // std.debug.print("HI I GOT CHARACTER: {c}", .{ destination[i] });
        if (destination[i] == '/') {
            if (std.mem.eql(u8, destination[0..i], source[0..i])) {
                std.debug.print("PATHS ARE EQUAL: {s} AND I IS {o}\n", .{destination[0..i], i});
            }
        }

        i += 1;
    }
    std.debug.print("------\n", .{});

    return "hello";

    // while (i < ) : (i += 1) {
    //     if (path[i-1] == '/') {
    //         output = path[0..i-1];
    //         break;
    //     }
    // }


}

pub fn get_parent_dir(path: []const u8) []const u8{

    var i: usize = path.len;
    var output = path;
    // loop in reverse and finds the first occurence of /
    while (i > 0) : (i -= 1) {
        if (path[i-1] == '/') {
            output = path[0..i-1];
            break;
        }
    }

    return output;
}
pub fn get_file_name(path: []const u8) []const u8 {
    var i: usize = path.len;
    var output = path;
    // loop in reverse and finds the first occurence of /
    while (i > 0) : (i -= 1) {
        if (path[i-1] == '/') {
            output = path[i..];
            break;
        }
    }

    return output;
}
