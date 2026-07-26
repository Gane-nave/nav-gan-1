/**
 * Tests for shared/contracts/debugLog.ts
 */
import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import {
  createLogger,
  log,
  setLogReporter,
  __setLogDevMode,
  type LogEvent,
} from "../../shared/contracts/debugLog";

let captured: LogEvent[] = [];
let consoleMock: ReturnType<typeof vi.spyOn>[] = [];

beforeEach(() => {
  captured = [];
  setLogReporter((ev) => captured.push(ev));
  consoleMock = [
    vi.spyOn(console, "debug").mockImplementation(() => {}),
    vi.spyOn(console, "info").mockImplementation(() => {}),
    vi.spyOn(console, "warn").mockImplementation(() => {}),
    vi.spyOn(console, "error").mockImplementation(() => {}),
  ];
});

afterEach(() => {
  setLogReporter(null);
  __setLogDevMode(null);
  consoleMock.forEach((m) => m.mockRestore());
});

describe("createLogger", () => {
  it("produces a scoped logger with all four levels", () => {
    const l = createLogger("mod");
    expect(typeof l.debug).toBe("function");
    expect(typeof l.info).toBe("function");
    expect(typeof l.warn).toBe("function");
    expect(typeof l.error).toBe("function");
  });

  it("tags events with the module name in dev", () => {
    __setLogDevMode(true);
    const l = createLogger("chaos");
    l.info("starting", { scenario: "A" });
    expect(console.info).toHaveBeenCalledWith("[chaos]", "starting", { scenario: "A" });
  });
});

describe("prod mode behaviour", () => {
  it("silences debug and info", () => {
    __setLogDevMode(false);
    log("mod", "debug", "ignored");
    log("mod", "info", "also ignored");
    expect(console.debug).not.toHaveBeenCalled();
    expect(console.info).not.toHaveBeenCalled();
  });

  it("forwards warn and error to the reporter", () => {
    __setLogDevMode(false);
    log("mod", "warn", "heads up");
    log("mod", "error", "broken", { code: 500 });
    expect(captured.map((e) => e.level)).toEqual(["warn", "error"]);
    expect(captured[1].meta).toEqual({ code: 500 });
  });

  it("still prints errors to console.error in prod", () => {
    __setLogDevMode(false);
    log("mod", "error", "boom");
    expect(console.error).toHaveBeenCalledWith("[mod]", "boom", "");
  });

  it("does not throw when the reporter throws", () => {
    __setLogDevMode(false);
    setLogReporter(() => {
      throw new Error("reporter exploded");
    });
    expect(() => log("mod", "error", "still fine")).not.toThrow();
  });
});

describe("dev mode behaviour", () => {
  it("routes to the matching console method", () => {
    __setLogDevMode(true);
    log("mod", "debug", "d");
    log("mod", "info", "i");
    log("mod", "warn", "w");
    log("mod", "error", "e");
    expect(console.debug).toHaveBeenCalled();
    expect(console.info).toHaveBeenCalled();
    expect(console.warn).toHaveBeenCalled();
    expect(console.error).toHaveBeenCalled();
  });
});
