/**
 * G.A.N.E — Encrypted Collab Transport Wrapper
 * ==============================================
 * Transport-agnostic layer that wraps any send/receive pair with optional
 * AES-GCM-256 payload encryption. Works with WebSocket, SSE, WebRTC
 * DataChannel, postMessage — anything whose message shape is JSON-ish.
 *
 * Contract:
 *   - If `sessionKey` is null, messages pass through unchanged.
 *   - If `sessionKey` is set, the `payload` field of each outgoing
 *     message is encrypted with encryptJSON and the envelope is marked
 *     with an `enc` discriminator so peers know to decrypt on receive.
 *   - Non-payload metadata (type, sessionId, userId, timestamp) stays in
 *     clear so relays can route/audit without the key.
 *
 * Pairs with shared/contracts/e2eEncryption.ts.
 */
import {
  decryptJSON,
  encryptJSON,
  type EncryptedPayload,
} from "./e2eEncryption";

export interface CollabMessage<P = Record<string, unknown>> {
  type: string;
  sessionId: string;
  userId: number;
  userName?: string;
  userColor?: string;
  payload: P;
  timestamp: number;
}

/** Message whose payload has been sealed by `encryptJSON`. */
export interface EncryptedCollabMessage
  extends Omit<CollabMessage<unknown>, "payload"> {
  enc: true;
  payload: EncryptedPayload;
}

export function isEncryptedCollabMessage(
  m: unknown,
): m is EncryptedCollabMessage {
  if (!m || typeof m !== "object") return false;
  const obj = m as Record<string, unknown>;
  if (obj.enc !== true) return false;
  const p = obj.payload;
  if (!p || typeof p !== "object") return false;
  const pp = p as Record<string, unknown>;
  return pp.alg === "AES-GCM-256" && pp.v === 1 && typeof pp.iv === "string";
}

export class EncryptedTransport<P = Record<string, unknown>> {
  private key: CryptoKey | null = null;

  setSessionKey(key: CryptoKey | null): void {
    this.key = key;
  }

  hasSessionKey(): boolean {
    return this.key !== null;
  }

  /**
   * Prepare an outgoing message for transport. Produces either the plain
   * envelope or an `EncryptedCollabMessage` ready to JSON.stringify.
   */
  async outbound(
    msg: CollabMessage<P>,
  ): Promise<CollabMessage<P> | EncryptedCollabMessage> {
    if (!this.key) return msg;
    const sealed = await encryptJSON(this.key, msg.payload);
    const { payload: _omit, ...rest } = msg;
    void _omit;
    return {
      ...rest,
      enc: true,
      payload: sealed,
    };
  }

  /**
   * Parse an incoming message. If it was encrypted and we hold the key,
   * return the decrypted CollabMessage. If encrypted but no key, throw.
   * If not encrypted, return as-is.
   */
  async inbound(
    msg: CollabMessage<P> | EncryptedCollabMessage | unknown,
  ): Promise<CollabMessage<P>> {
    if (isEncryptedCollabMessage(msg)) {
      if (!this.key) {
        throw new Error(
          "EncryptedTransport.inbound: received encrypted message but no session key is set",
        );
      }
      const payload = await decryptJSON<P>(this.key, msg.payload);
      const { enc: _e, payload: _p, ...rest } = msg;
      void _e;
      void _p;
      return { ...rest, payload };
    }
    // Trust-but-verify: at this point the caller has handed us something
    // it believes is a CollabMessage. We do a cheap shape check and
    // throw on obvious garbage so callers get a useful error.
    if (!msg || typeof msg !== "object") {
      throw new Error("EncryptedTransport.inbound: message is not an object");
    }
    const obj = msg as CollabMessage<P>;
    if (typeof obj.type !== "string" || typeof obj.sessionId !== "string") {
      throw new Error(
        "EncryptedTransport.inbound: missing required fields (type, sessionId)",
      );
    }
    return obj;
  }
}
