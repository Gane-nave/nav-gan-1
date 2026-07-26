/**
 * Tests for shared/contracts/collabTransport.ts
 * Uses the WebCrypto primitives from e2eEncryption.ts end-to-end.
 */
import { describe, it, expect } from "vitest";
import {
  EncryptedTransport,
  isEncryptedCollabMessage,
  type CollabMessage,
} from "../../shared/contracts/collabTransport";
import {
  generateSessionKey,
  exportSessionKey,
  importSessionKey,
} from "../../shared/contracts/e2eEncryption";

interface Payload {
  lat: number;
  lon: number;
  note?: string;
}

function makeMessage(override: Partial<CollabMessage<Payload>> = {}): CollabMessage<Payload> {
  return {
    type: "cursor",
    sessionId: "sess-1",
    userId: 42,
    userName: "alice",
    userColor: "#22c55e",
    payload: { lat: 32.08, lon: 34.78 },
    timestamp: 1700000000000,
    ...override,
  };
}

describe("EncryptedTransport", () => {
  it("passes messages through unchanged when no session key is set", async () => {
    const t = new EncryptedTransport<Payload>();
    expect(t.hasSessionKey()).toBe(false);
    const msg = makeMessage();
    const outbound = await t.outbound(msg);
    expect(isEncryptedCollabMessage(outbound)).toBe(false);
    expect(outbound).toEqual(msg);
    const inbound = await t.inbound(outbound);
    expect(inbound).toEqual(msg);
  });

  it("encrypts payload but leaves metadata in clear when a key is set", async () => {
    const t = new EncryptedTransport<Payload>();
    t.setSessionKey(await generateSessionKey());
    const msg = makeMessage({ payload: { lat: 1, lon: 2, note: "hidden" } });
    const outbound = await t.outbound(msg);
    expect(isEncryptedCollabMessage(outbound)).toBe(true);
    if (isEncryptedCollabMessage(outbound)) {
      expect(outbound.type).toBe("cursor");
      expect(outbound.sessionId).toBe("sess-1");
      expect(outbound.userId).toBe(42);
      expect(outbound.payload.alg).toBe("AES-GCM-256");
      // ciphertext is base64url — ensure it doesn't leak "hidden"
      expect(outbound.payload.ciphertext).not.toMatch(/hidden/);
    }
  });

  it("round-trips across two peers that share an exported key", async () => {
    const alice = new EncryptedTransport<Payload>();
    const bob = new EncryptedTransport<Payload>();
    const key = await generateSessionKey();
    const exported = await exportSessionKey(key);
    alice.setSessionKey(key);
    bob.setSessionKey(await importSessionKey(exported));

    const msg = makeMessage({ payload: { lat: 10, lon: 20, note: "meet at the cafe" } });
    const wireFormat = JSON.stringify(await alice.outbound(msg));
    const received = await bob.inbound(JSON.parse(wireFormat));
    expect(received).toEqual(msg);
  });

  it("throws when decrypting without a key", async () => {
    const sender = new EncryptedTransport<Payload>();
    const receiver = new EncryptedTransport<Payload>();
    sender.setSessionKey(await generateSessionKey());
    // receiver has no key
    const sealed = await sender.outbound(makeMessage());
    await expect(receiver.inbound(sealed)).rejects.toThrow(/no session key/);
  });

  it("throws on structurally invalid inbound messages", async () => {
    const t = new EncryptedTransport<Payload>();
    await expect(t.inbound(null)).rejects.toThrow(/not an object/);
    await expect(t.inbound({ type: "cursor" })).rejects.toThrow(/missing required fields/);
  });

  it("isEncryptedCollabMessage rejects plain envelopes", () => {
    expect(isEncryptedCollabMessage(makeMessage())).toBe(false);
    expect(isEncryptedCollabMessage({})).toBe(false);
    expect(isEncryptedCollabMessage(null)).toBe(false);
    expect(isEncryptedCollabMessage(undefined)).toBe(false);
  });
});
