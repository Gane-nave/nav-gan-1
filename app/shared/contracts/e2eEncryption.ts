/**
 * G.A.N.E — End-to-End Encryption for Collaboration
 * ====================================================
 * Thin, testable wrapper over the Web Crypto API (SubtleCrypto) to let the
 * realtime-collab engine encrypt DataChannel / SSE payloads with AES-GCM.
 *
 * Covers todo.md:
 *   - [x] End-to-end encryption for secure sharing
 *
 * Key agreement itself (ECDH via WebRTC signaling) is out of scope here:
 * this module provides the symmetric-key primitives a key-exchange layer
 * will use on top. Pure enough to run under Node 19+ (which exposes
 * `crypto.subtle` globally) and in every browser we target.
 */

export interface EncryptedPayload {
  /** IV used for this message · 12 bytes as base64url. */
  iv: string;
  /** AES-GCM ciphertext as base64url. */
  ciphertext: string;
  /** Optional additional authenticated data (base64url). */
  aad?: string;
  /** Algorithm + version marker so we can rotate later. */
  alg: "AES-GCM-256";
  v: 1;
}

const SUBTLE: SubtleCrypto = (globalThis as unknown as { crypto: { subtle: SubtleCrypto } }).crypto.subtle;
const RNG = (globalThis as unknown as { crypto: Crypto }).crypto;

const AES_ALG = { name: "AES-GCM", length: 256 } as const;
const IV_BYTES = 12;

/**
 * Copy into a freshly-allocated ArrayBuffer so the result satisfies the
 * tightened `BufferSource` = `ArrayBufferView<ArrayBuffer>` contract in
 * current @types/node and lib.dom. TypedArrays produced by TextEncoder
 * or derived from base64 helpers are typed with `ArrayBufferLike` and
 * can't be passed to WebCrypto without a copy.
 */
function toAB(u: Uint8Array): ArrayBuffer {
  const ab = new ArrayBuffer(u.byteLength);
  new Uint8Array(ab).set(u);
  return ab;
}

// ── base64url helpers (RFC 4648 §5, no padding) ───────────────────────────

export function bytesToB64(bytes: Uint8Array): string {
  let bin = "";
  for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i]);
  const b64 = (globalThis as unknown as { btoa: (s: string) => string }).btoa(bin);
  return b64.replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

export function b64ToBytes(b64: string): Uint8Array {
  const normalized = b64.replace(/-/g, "+").replace(/_/g, "/");
  const pad = normalized.length % 4 === 0 ? "" : "=".repeat(4 - (normalized.length % 4));
  const bin = (globalThis as unknown as { atob: (s: string) => string }).atob(normalized + pad);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

// ── Keys ────────────────────────────────────────────────────────────────

/** Generate a fresh AES-256-GCM session key. */
export async function generateSessionKey(): Promise<CryptoKey> {
  return SUBTLE.generateKey(AES_ALG, true, ["encrypt", "decrypt"]);
}

/** Export a key to a base64url-encoded raw byte string (for persistence). */
export async function exportSessionKey(key: CryptoKey): Promise<string> {
  const raw = await SUBTLE.exportKey("raw", key);
  return bytesToB64(new Uint8Array(raw));
}

/** Import a key previously emitted by exportSessionKey. */
export async function importSessionKey(b64: string): Promise<CryptoKey> {
  const raw = b64ToBytes(b64);
  return SUBTLE.importKey("raw", toAB(raw), AES_ALG, true, ["encrypt", "decrypt"]);
}

/**
 * Derive a session key from a shared passphrase via PBKDF2-SHA-256
 * (210 000 iterations · OWASP 2023 baseline). Intended for demo /
 * fallback scenarios where an explicit key-agreement step has not yet
 * been established; real deployments should use ECDH via signalling.
 */
export async function deriveSessionKeyFromPassphrase(
  passphrase: string,
  salt: Uint8Array,
  iterations = 210_000,
): Promise<CryptoKey> {
  const encoder = new TextEncoder();
  const baseKey = await SUBTLE.importKey(
    "raw",
    toAB(encoder.encode(passphrase)),
    { name: "PBKDF2" },
    false,
    ["deriveKey"],
  );
  return SUBTLE.deriveKey(
    { name: "PBKDF2", hash: "SHA-256", salt: toAB(salt), iterations },
    baseKey,
    AES_ALG,
    true,
    ["encrypt", "decrypt"],
  );
}

// ── Encrypt / Decrypt ─────────────────────────────────────────────────────

export interface EncryptOptions {
  /** Additional authenticated data — not encrypted, but signed. */
  aad?: Uint8Array;
}

export async function encryptPayload(
  key: CryptoKey,
  plaintext: Uint8Array | string,
  opts: EncryptOptions = {},
): Promise<EncryptedPayload> {
  const iv = new Uint8Array(IV_BYTES);
  RNG.getRandomValues(iv);
  const data =
    typeof plaintext === "string" ? new TextEncoder().encode(plaintext) : plaintext;
  const cipher = await SUBTLE.encrypt(
    {
      name: "AES-GCM",
      iv: toAB(iv),
      additionalData: opts.aad ? toAB(opts.aad) : undefined,
    },
    key,
    toAB(data),
  );
  const out: EncryptedPayload = {
    iv: bytesToB64(iv),
    ciphertext: bytesToB64(new Uint8Array(cipher)),
    alg: "AES-GCM-256",
    v: 1,
  };
  if (opts.aad) out.aad = bytesToB64(opts.aad);
  return out;
}

export async function decryptPayload(
  key: CryptoKey,
  payload: EncryptedPayload,
): Promise<Uint8Array> {
  if (payload.alg !== "AES-GCM-256" || payload.v !== 1) {
    throw new Error(`Unsupported envelope: alg=${payload.alg} v=${payload.v}`);
  }
  const iv = b64ToBytes(payload.iv);
  const ciphertext = b64ToBytes(payload.ciphertext);
  const aad = payload.aad ? b64ToBytes(payload.aad) : undefined;
  const plain = await SUBTLE.decrypt(
    {
      name: "AES-GCM",
      iv: toAB(iv),
      additionalData: aad ? toAB(aad) : undefined,
    },
    key,
    toAB(ciphertext),
  );
  return new Uint8Array(plain);
}

/** Convenience: encrypt a JSON-serialisable value. */
export async function encryptJSON<T>(
  key: CryptoKey,
  value: T,
  opts?: EncryptOptions,
): Promise<EncryptedPayload> {
  return encryptPayload(key, JSON.stringify(value), opts);
}

/** Convenience: decrypt into a JSON-parsed value. */
export async function decryptJSON<T>(
  key: CryptoKey,
  payload: EncryptedPayload,
): Promise<T> {
  const bytes = await decryptPayload(key, payload);
  const text = new TextDecoder().decode(bytes);
  return JSON.parse(text) as T;
}
