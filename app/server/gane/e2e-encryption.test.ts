/**
 * Tests for shared/contracts/e2eEncryption.ts
 * Exercises round-trip AES-GCM, AAD binding, wrong-key rejection, base64url
 * helpers, and PBKDF2 passphrase derivation.
 *
 * Runs under Node 19+ which exposes `crypto.subtle` globally.
 */
import { describe, it, expect } from "vitest";
import {
  b64ToBytes,
  bytesToB64,
  decryptJSON,
  decryptPayload,
  deriveSessionKeyFromPassphrase,
  encryptJSON,
  encryptPayload,
  exportSessionKey,
  generateSessionKey,
  importSessionKey,
} from "../../shared/contracts/e2eEncryption";

describe("base64url helpers", () => {
  it("round-trips arbitrary bytes", () => {
    const bytes = new Uint8Array([0, 1, 2, 250, 251, 252, 253, 254, 255]);
    const b64 = bytesToB64(bytes);
    expect(b64).not.toMatch(/[+/=]/);
    expect(Array.from(b64ToBytes(b64))).toEqual(Array.from(bytes));
  });
});

describe("session key lifecycle", () => {
  it("generates and round-trips through export/import", async () => {
    const k1 = await generateSessionKey();
    const exported = await exportSessionKey(k1);
    const k2 = await importSessionKey(exported);
    const payload = await encryptPayload(k1, "hello");
    const plain = await decryptPayload(k2, payload);
    expect(new TextDecoder().decode(plain)).toBe("hello");
  });
});

describe("encrypt / decrypt", () => {
  it("encrypts and decrypts strings with AES-GCM-256", async () => {
    const key = await generateSessionKey();
    const payload = await encryptPayload(key, "top-secret position fix");
    expect(payload.alg).toBe("AES-GCM-256");
    expect(payload.v).toBe(1);
    expect(payload.iv).toBeTruthy();
    expect(payload.ciphertext).toBeTruthy();
    const plain = await decryptPayload(key, payload);
    expect(new TextDecoder().decode(plain)).toBe("top-secret position fix");
  });

  it("produces a unique IV per encryption", async () => {
    const key = await generateSessionKey();
    const a = await encryptPayload(key, "same message");
    const b = await encryptPayload(key, "same message");
    expect(a.iv).not.toBe(b.iv);
    expect(a.ciphertext).not.toBe(b.ciphertext);
  });

  it("rejects decryption with the wrong key", async () => {
    const k1 = await generateSessionKey();
    const k2 = await generateSessionKey();
    const payload = await encryptPayload(k1, "classified");
    await expect(decryptPayload(k2, payload)).rejects.toBeDefined();
  });

  it("authenticates associated data (AAD)", async () => {
    const key = await generateSessionKey();
    const aad = new TextEncoder().encode("session-42");
    const payload = await encryptPayload(key, "payload", { aad });
    // Decryption should fail if we tamper with the AAD field.
    const tampered = { ...payload, aad: bytesToB64(new TextEncoder().encode("session-99")) };
    await expect(decryptPayload(key, tampered)).rejects.toBeDefined();
  });

  it("rejects unsupported envelope version/alg", async () => {
    const key = await generateSessionKey();
    const payload = await encryptPayload(key, "x");
    const bad = { ...payload, v: 99 } as unknown as typeof payload;
    await expect(decryptPayload(key, bad)).rejects.toThrow(/Unsupported envelope/);
  });
});

describe("JSON helpers", () => {
  it("round-trips a typed JSON object", async () => {
    const key = await generateSessionKey();
    interface Shape { user: string; lat: number; lon: number; }
    const input: Shape = { user: "alice", lat: 32.08, lon: 34.78 };
    const payload = await encryptJSON(key, input);
    const output = await decryptJSON<Shape>(key, payload);
    expect(output).toEqual(input);
  });
});

describe("PBKDF2 passphrase derivation", () => {
  it("derives the same key from the same passphrase+salt", async () => {
    const salt = new Uint8Array(16).fill(7);
    const k1 = await deriveSessionKeyFromPassphrase("correct horse battery", salt, 50_000);
    const k2 = await deriveSessionKeyFromPassphrase("correct horse battery", salt, 50_000);
    const payload = await encryptPayload(k1, "inter-peer message");
    const plain = await decryptPayload(k2, payload);
    expect(new TextDecoder().decode(plain)).toBe("inter-peer message");
  });
});
