const std = @import("std");

const EnsureInputErrors = error{
    BadStatus,
};

fn ensure_input(a: std.mem.Allocator, day: u8) EnsureInputErrors {
    var threaded: std.Io.Threaded = .init(a, std.Io.Threaded.InitOptions{});
    defer threaded.deinit();

    const io = threaded.io();

    var c = std.http.Client{
        .allocator = a,
        .io = io,
    };

    const bytes: std.ArrayList(u8) = .empty;
    defer bytes.deinit(a);

    const u: []const u8 = "https://adventofcode.com/2025/day/" ++ .{ day } ++ "/input";

    const res = c.fetch(std.http.Client.FetchOptions{
        .location = .{ .url = u },
        .response_writer = bytes,
    }) catch 0;
    if (res.status != 200) {
        return EnsureInputErrors.BadStatus;
    }

    std.debug.print("", bytes.items);
}

pub fn main() void {
    var a = std.heap.page_allocator;
    defer a.deinit();

    ensure_input(a, 1) catch noreturn;

    std.debug.print("Hello, {s}!\n", .{"World"});
}
