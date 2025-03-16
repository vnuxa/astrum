pub const DocDefine = struct {
    file: ?[]const u8 = null,
    // type: []const u8,
    view: []const u8,
    // visible: []const u8,
};
pub const DocExtendArg = struct {
    name: []const u8,
    // type: []const u8,
    view: []const u8,
    desc: ?[]const u8 = null
};
pub const DocReturn = struct {
    type: ?[]const u8 = null,
    view: []const u8,
};
pub const DocExtendField = struct {
    type: []const u8,
    view: []const u8,
    name: ?[]const u8 = null,
    desc: ?[]const u8 = null,
    args: ?[]DocExtendArg = null,
    returns: ?[]DocReturn = null
};
pub const DocField = struct {
    desc: ?[]const u8,
    file: ?[]const u8 = null,
    type: []const u8,
    view: []const u8,
    // visible: []const u8,
    name: []const u8,
    extends: DocExtendField
};
pub const DocObject = struct {
    defines: []DocDefine,
    fields: []DocField,
};
