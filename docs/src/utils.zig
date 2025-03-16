const std = @import("std");
const main = @import("main.zig");
const json = @import("json_defines.zig");
const epoch = std.time.epoch;


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
        // TODO: improve definition detection
        if (std.mem.containsAtLeast(u8, proprety.view, 1, entry.key_ptr.*)) {
            const value: main.DefinitionObject = entry.value_ptr.*;
            // std.debug.print("--sasme name?: {s} in proprety: {s}\n", .{object_name, proprety.name});
            // std.debug.print("!!!!!!!!!!!! value: {s} ||||| mdfile: {s}\n", .{value.path, mdfile_name});
            // std.debug.print("yeah cut away is: {s}\n", .{mdfile_name[16..mdfile_name.len-3]});
            const relative_path: ?[]const u8 = blk: {
                // std.debug.print("mem length: {d} || and mdfile is {s}", .{ mdfile_name.len, mdfile_name});
                if (std.mem.eql(u8, value.path, mdfile_name[16..mdfile_name.len-3])) {
                    break :blk "";
                }
                const parent_value = get_parent_dir(value.path);
                if (parent_value.len != 0 and std.mem.containsAtLeast(u8, mdfile_name[16..], 1, parent_value)) {
                    break :blk try std.fmt.allocPrint(allocator, "./{s}.md", .{get_file_name(value.path)});
                }
                if (std.mem.eql(u8, mdfile_dir, "./main/auto_gen")) {
                    break :blk try std.fmt.allocPrint(allocator, "./{s}.md", .{value.path});
                }
                // break :blk try std.fmt.allocPrint(allocator, "(wow {s} || par length: {s})", .{mdfile_dir, mdfile_dir});
                break :blk try std.fmt.allocPrint(allocator, "../{s}.md", .{value.path});
            };


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
    if (std.mem.eql(u8, path, output)) {
        std.debug.print("the output is same as input", .{});
        return "";
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
pub fn get_parent_name(path: []const u8) []const u8 {
    var i: usize = path.len;
    var output = path;
    var found = 0;
    // loop in reverse and finds the first occurence of /
    while (i > 0) : (i -= 1) {
        if (path[i-1] == '/') {
            if (found != 0) {
                output = path[i-1..found];
                break;
            }
            found = i;
            continue;
        }

    }

    return output;
}



pub fn current_date(allocator: std.mem.Allocator) ![]const u8 {

    // var string: [24]u8 = undefined;
    const current_datetime = epoch.EpochSeconds.getEpochDay(epoch.EpochSeconds {
        .secs = @intCast(std.time.timestamp()),
        // .secs =  timestamp
    });
    const year = current_datetime.calculateYearDay();
    const month = year.calculateMonthDay();

    if (month.month.numeric() < 10 ) {
        return try std.fmt.allocPrint(allocator, "{d}-0{d}-{d}", .{ year.year, month.month.numeric(), month.day_index });
        // return &string;
    }

    return try std.fmt.allocPrint(allocator, "{d}-{d}-{d}", .{ year.year, month.month.numeric(), month.day_index });
    // _ = try std.fmt.format(&string, "{d}-{d}-{d}", .{ year.year, month.month.numeric(), month.day_index });
    // return &string;
    // return "" ++ year.year ++ month.month.numeric() ++ "-" ++ month.day_index();
}
