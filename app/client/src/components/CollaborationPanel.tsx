/**
 * CollaborationPanel — Real-Time Multi-User Map Editing
 * ======================================================
 * Full collaboration interface with:
 * - Session creation/joining
 * - Live participant list with colored indicators
 * - Shared marker management
 * - Activity feed
 * - Connection status
 */
import { useState, useMemo } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Users, Plus, X, MapPin, MessageSquare, Activity,
  Wifi, WifiOff, Copy, Check, Trash2, ArrowRight,
  Circle, UserPlus, LogOut, Share2, Link2
} from "lucide-react";
import { useCollaboration } from "@/hooks/useCollaboration";
import { trpc } from "@/lib/trpc";
import { useAuth } from "@/_core/hooks/useAuth";

type Tab = "participants" | "markers" | "activity";

const COLORS = {
  cyan: "#00e5ff",
  green: "#00ff88",
  red: "#ff3355",
  purple: "#aa66ff",
  orange: "#ff9900",
  gold: "#ffd700",
};

export default function CollaborationPanel({
  onClose,
  onMarkerClick,
}: {
  onClose: () => void;
  onMarkerClick?: (lat: number, lon: number) => void;
}) {
  const { user } = useAuth();
  const [activeTab, setActiveTab] = useState<Tab>("participants");
  const [activeSessionId, setActiveSessionId] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [newSessionName, setNewSessionName] = useState("");
  const [newMarkerLabel, setNewMarkerLabel] = useState("");
  const [copiedId, setCopiedId] = useState(false);
  const [showShareModal, setShowShareModal] = useState(false);
  const [inviteLink, setInviteLink] = useState<string | null>(null);
  const [copiedLink, setCopiedLink] = useState(false);
  const [generatingInvite, setGeneratingInvite] = useState(false);
  const generateInviteMutation = trpc.collaboration.generateInvite.useMutation();

  const sessionsQuery = trpc.collaboration.listSessions.useQuery(undefined, {
    refetchInterval: 10000,
  });
  const createSessionMutation = trpc.collaboration.createSession.useMutation();

  const {
    isConnected,
    participants,
    markers,
    events,
    joinSession,
    leaveSession,
    addMarker,
    deleteMarker,
  } = useCollaboration(activeSessionId);

  // ─── Create Session ───
  const handleCreateSession = async () => {
    if (!newSessionName.trim()) return;
    try {
      const result = await createSessionMutation.mutateAsync({
        name: newSessionName.trim(),
      });
      setNewSessionName("");
      setShowCreateForm(false);
      // Auto-join the created session
      await joinSession(result.sessionId);
      setActiveSessionId(result.sessionId);
      sessionsQuery.refetch();
    } catch (err) {
      console.error("Failed to create session:", err);
    }
  };

  // ─── Join Existing Session ───
  const handleJoinSession = async (sessionId: string) => {
    try {
      await joinSession(sessionId);
      setActiveSessionId(sessionId);
    } catch (err) {
      console.error("Failed to join session:", err);
    }
  };

  // ─── Leave Session ───
  const handleLeaveSession = async () => {
    await leaveSession();
    setActiveSessionId(null);
  };

  // ─── Copy Session ID ───
  const handleCopyId = () => {
    if (activeSessionId) {
      navigator.clipboard.writeText(activeSessionId);
      setCopiedId(true);
      setTimeout(() => setCopiedId(false), 2000);
    }
  };

  // ─── Generate Invite Link ───
  const handleGenerateInvite = async () => {
    if (!activeSessionId) return;
    setGeneratingInvite(true);
    try {
      const result = await generateInviteMutation.mutateAsync({
        sessionId: activeSessionId,
        expiresInHours: 24,
      });
      const link = `${window.location.origin}/collab/join/${result.inviteToken}`;
      setInviteLink(link);
      setShowShareModal(true);
    } catch (err) {
      console.error("Failed to generate invite:", err);
    } finally {
      setGeneratingInvite(false);
    }
  };

  // ─── Copy Invite Link ───
  const handleCopyInviteLink = () => {
    if (inviteLink) {
      navigator.clipboard.writeText(inviteLink);
      setCopiedLink(true);
      setTimeout(() => setCopiedLink(false), 2000);
    }
  };

  // ─── Add Marker at Map Center ───
  const handleAddMarker = async () => {
    // Default to Tel Aviv center — in production, pass current map center
    await addMarker(32.0853, 34.7818, newMarkerLabel || "New Marker", undefined, COLORS.cyan);
    setNewMarkerLabel("");
  };

  // ─── Format Event ───
  const formatEvent = (event: { type: string; payload: Record<string, unknown>; timestamp: number }) => {
    const time = new Date(event.timestamp).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    const typeLabels: Record<string, string> = {
      user_joined: "joined the session",
      user_left: "left the session",
      marker_added: "added a marker",
      marker_moved: "moved a marker",
      marker_deleted: "removed a marker",
      annotation_added: "added an annotation",
      annotation_deleted: "removed an annotation",
      session_ended: "ended the session",
    };
    return { time, label: typeLabels[event.type] || event.type };
  };

  // ─── Active Session ───
  const activeSession = useMemo(() => {
    const rawData = sessionsQuery.data;
    const sessions = rawData && typeof rawData === 'object' && 'sessions' in rawData ? (rawData as { sessions: Array<{ sessionId: string; name: string; participantCount: number; createdAt: Date }> }).sessions : (rawData as Array<{ sessionId: string; name: string; participantCount: number; createdAt: Date }> | undefined);
    return sessions?.find((s: any) => s.sessionId === activeSessionId);
  }, [sessionsQuery.data, activeSessionId]);

  // ═══════════════════════════════════════════════════
  // LOBBY VIEW — Session List
  // ═══════════════════════════════════════════════════
  if (!activeSessionId) {
    return (
      <motion.div
        initial={{ x: 300, opacity: 0 }}
        animate={{ x: 0, opacity: 1 }}
        exit={{ x: 300, opacity: 0 }}
        className="absolute right-0 top-0 bottom-0 w-[380px] z-[60] flex flex-col"
        style={{
          background: "rgba(2, 6, 23, 0.95)",
          borderLeft: `1px solid rgba(0, 229, 255, 0.15)`,
          backdropFilter: "blur(20px)",
        }}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-5 py-4 border-b border-white/10">
          <div className="flex items-center gap-3">
            <Users size={20} style={{ color: COLORS.cyan }} />
            <span className="text-white font-semibold text-base">Collaboration</span>
          </div>
          <button onClick={onClose} className="text-white/50 hover:text-white transition-colors">
            <X size={18} />
          </button>
        </div>

        {/* Create Session */}
        <div className="px-5 py-4 border-b border-white/10">
          {showCreateForm ? (
            <div className="flex flex-col gap-3">
              <input
                type="text"
                value={newSessionName}
                onChange={(e) => setNewSessionName(e.target.value)}
                placeholder="Session name..."
                className="w-full px-3 py-2 rounded-lg bg-white/5 border border-white/10 text-white text-sm placeholder:text-white/30 focus:outline-none focus:border-cyan-400/50"
                onKeyDown={(e) => e.key === "Enter" && handleCreateSession()}
                autoFocus
              />
              <div className="flex gap-2">
                <button
                  onClick={handleCreateSession}
                  disabled={!newSessionName.trim()}
                  className="flex-1 px-3 py-2 rounded-lg text-sm font-medium transition-all disabled:opacity-30"
                  style={{ background: COLORS.cyan, color: "#020617" }}
                >
                  Create
                </button>
                <button
                  onClick={() => setShowCreateForm(false)}
                  className="px-3 py-2 rounded-lg text-sm text-white/60 hover:text-white bg-white/5 hover:bg-white/10 transition-all"
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <button
              onClick={() => setShowCreateForm(true)}
              className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-all hover:brightness-110"
              style={{ background: `linear-gradient(135deg, ${COLORS.cyan}, ${COLORS.purple})`, color: "#020617" }}
            >
              <Plus size={16} />
              New Session
            </button>
          )}
        </div>

        {/* Session List */}
        <div className="flex-1 overflow-y-auto px-5 py-3 space-y-3 custom-scrollbar">
          {sessionsQuery.isLoading && (
            <div className="text-center text-white/40 py-8 text-sm">Loading sessions...</div>
          )}
          {(sessionsQuery.data && 'sessions' in sessionsQuery.data ? sessionsQuery.data.sessions : sessionsQuery.data)?.length === 0 && (
            <div className="text-center text-white/40 py-8 text-sm">
              No active sessions. Create one to start collaborating.
            </div>
          )}
          {(sessionsQuery.data && typeof sessionsQuery.data === 'object' && 'sessions' in sessionsQuery.data ? (sessionsQuery.data as { sessions: Array<Record<string, unknown>> }).sessions : (sessionsQuery.data as Array<Record<string, unknown>> | undefined))?.map((session: any) => (
            <motion.div
              key={session.sessionId}
              className="p-4 rounded-xl border border-white/10 hover:border-cyan-400/30 transition-all cursor-pointer"
              style={{ background: "rgba(255,255,255,0.03)" }}
              whileHover={{ scale: 1.01 }}
              onClick={() => handleJoinSession(session.sessionId)}
            >
              <div className="flex items-center justify-between mb-2">
                <span className="text-white font-medium text-sm">{session.name}</span>
                <div className="flex items-center gap-1.5">
                  <Circle size={8} fill={COLORS.green} stroke="none" />
                  <span className="text-white/50 text-xs">{session.participantCount} online</span>
                </div>
              </div>
              <div className="flex items-center gap-1.5">
                {session.participants.slice(0, 5).map((p: any) => (
                  <div
                    key={p.userId}
                    className="w-6 h-6 rounded-full flex items-center justify-center text-[10px] font-bold text-white"
                    style={{ background: p.color }}
                    title={p.displayName || undefined}
                  >
                    {(p.displayName || "?")[0].toUpperCase()}
                  </div>
                ))}
                {session.participantCount > 5 && (
                  <span className="text-white/40 text-xs ml-1">+{session.participantCount - 5}</span>
                )}
              </div>
              <div className="flex items-center gap-2 mt-2">
                <ArrowRight size={14} style={{ color: COLORS.cyan }} />
                <span className="text-xs" style={{ color: COLORS.cyan }}>
                  Join Session
                </span>
              </div>
            </motion.div>
          ))}
        </div>
      </motion.div>
    );
  }

  // ═══════════════════════════════════════════════════
  // ACTIVE SESSION VIEW
  // ═══════════════════════════════════════════════════
  const tabs: { id: Tab; icon: typeof Users; label: string }[] = [
    { id: "participants", icon: Users, label: "Users" },
    { id: "markers", icon: MapPin, label: "Markers" },
    { id: "activity", icon: Activity, label: "Feed" },
  ];

  return (
    <motion.div
      initial={{ x: 300, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: 300, opacity: 0 }}
      className="absolute right-0 top-0 bottom-0 w-[380px] z-[60] flex flex-col"
      style={{
        background: "rgba(2, 6, 23, 0.95)",
        borderLeft: `1px solid rgba(0, 229, 255, 0.15)`,
        backdropFilter: "blur(20px)",
      }}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-3 border-b border-white/10">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            {isConnected ? (
              <Wifi size={16} style={{ color: COLORS.green }} />
            ) : (
              <WifiOff size={16} style={{ color: COLORS.red }} />
            )}
            <span className="text-white font-semibold text-sm truncate max-w-[180px]">
              {activeSession?.name || "Session"}
            </span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={handleGenerateInvite}
            disabled={generatingInvite}
            className="p-1.5 rounded-lg text-white/40 hover:text-cyan-400 hover:bg-white/10 transition-all"
            title="Share invite link"
          >
            <Share2 size={14} />
          </button>
          <button
            onClick={handleCopyId}
            className="p-1.5 rounded-lg text-white/40 hover:text-white hover:bg-white/10 transition-all"
            title="Copy session ID"
          >
            {copiedId ? <Check size={14} style={{ color: COLORS.green }} /> : <Copy size={14} />}
          </button>
          <button
            onClick={handleLeaveSession}
            className="p-1.5 rounded-lg text-white/40 hover:text-red-400 hover:bg-white/10 transition-all"
            title="Leave session"
          >
            <LogOut size={14} />
          </button>
          <button onClick={onClose} className="text-white/40 hover:text-white transition-colors">
            <X size={18} />
          </button>
        </div>
      </div>

      {/* Share Modal */}
      <AnimatePresence>
        {showShareModal && inviteLink && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            className="px-5 py-3 border-b border-white/10"
            style={{ background: 'rgba(0,229,255,0.05)' }}
          >
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <Link2 size={14} style={{ color: COLORS.cyan }} />
                <span className="text-white/80 text-xs font-medium">Invite Link (24h)</span>
              </div>
              <button
                onClick={() => setShowShareModal(false)}
                className="text-white/30 hover:text-white text-xs"
              >
                <X size={12} />
              </button>
            </div>
            <div className="flex gap-2">
              <input
                type="text"
                value={inviteLink}
                readOnly
                className="flex-1 px-2 py-1.5 rounded-lg bg-white/5 border border-white/10 text-white/70 text-xs truncate"
              />
              <button
                onClick={handleCopyInviteLink}
                className="px-3 py-1.5 rounded-lg text-xs font-medium transition-all"
                style={{
                  background: copiedLink ? COLORS.green : COLORS.cyan,
                  color: '#020617',
                }}
              >
                {copiedLink ? 'Copied!' : 'Copy'}
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Connection Status Bar */}
      <div
        className="px-5 py-2 text-xs flex items-center gap-2"
        style={{
          background: isConnected ? "rgba(0,255,136,0.05)" : "rgba(255,51,85,0.05)",
          borderBottom: `1px solid ${isConnected ? "rgba(0,255,136,0.1)" : "rgba(255,51,85,0.1)"}`,
        }}
      >
        <Circle size={6} fill={isConnected ? COLORS.green : COLORS.red} stroke="none" />
        <span style={{ color: isConnected ? COLORS.green : COLORS.red }}>
          {isConnected ? "Connected" : "Reconnecting..."}
        </span>
        <span className="text-white/30 ml-auto">
          {participants.length} participant{participants.length !== 1 ? "s" : ""}
        </span>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-white/10">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className="flex-1 flex items-center justify-center gap-1.5 py-3 text-xs font-medium transition-all relative"
            style={{
              color: activeTab === tab.id ? COLORS.cyan : "rgba(255,255,255,0.4)",
            }}
          >
            <tab.icon size={14} />
            {tab.label}
            {activeTab === tab.id && (
              <motion.div
                layoutId="collab-tab-indicator"
                className="absolute bottom-0 left-2 right-2 h-0.5 rounded-full"
                style={{ background: COLORS.cyan }}
              />
            )}
          </button>
        ))}
      </div>

      {/* Tab Content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar">
        <AnimatePresence mode="wait">
          {/* ─── Participants Tab ─── */}
          {activeTab === "participants" && (
            <motion.div
              key="participants"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="p-4 space-y-2"
            >
              {Array.isArray(participants) && participants.map((p) => (
                <div
                  key={p.userId}
                  className="flex items-center gap-3 p-3 rounded-xl border border-white/5"
                  style={{ background: "rgba(255,255,255,0.02)" }}
                >
                  <div
                    className="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold text-white"
                    style={{ background: p.color, boxShadow: `0 0 12px ${p.color}40` }}
                  >
                    {(p.displayName || "?")[0].toUpperCase()}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-white text-sm font-medium truncate">
                      {p.displayName || `User ${p.userId}`}
                      {p.userId === user?.id && (
                        <span className="text-white/30 text-xs ml-2">(you)</span>
                      )}
                    </div>
                    <div className="text-white/30 text-xs">
                      {p.cursorLat && p.cursorLon
                        ? `${p.cursorLat.toFixed(4)}, ${p.cursorLon.toFixed(4)}`
                        : "No cursor position"}
                    </div>
                  </div>
                  <Circle
                    size={8}
                    fill={p.isOnline ? COLORS.green : COLORS.red}
                    stroke="none"
                  />
                </div>
              ))}
              {(!participants || participants.length === 0) && (
                <div className="text-center text-white/30 py-8 text-sm">
                  <UserPlus size={24} className="mx-auto mb-2 opacity-30" />
                  No participants yet
                </div>
              )}
            </motion.div>
          )}

          {/* ─── Markers Tab ─── */}
          {activeTab === "markers" && (
            <motion.div
              key="markers"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="p-4 space-y-3"
            >
              {/* Add Marker Form */}
              <div className="flex gap-2">
                <input
                  type="text"
                  value={newMarkerLabel}
                  onChange={(e) => setNewMarkerLabel(e.target.value)}
                  placeholder="Marker label..."
                  className="flex-1 px-3 py-2 rounded-lg bg-white/5 border border-white/10 text-white text-sm placeholder:text-white/30 focus:outline-none focus:border-cyan-400/50"
                  onKeyDown={(e) => e.key === "Enter" && handleAddMarker()}
                />
                <button
                  onClick={handleAddMarker}
                  className="px-3 py-2 rounded-lg text-sm font-medium"
                  style={{ background: COLORS.cyan, color: "#020617" }}
                >
                  <Plus size={16} />
                </button>
              </div>

              {/* Marker List */}
              {markers.map((marker: any) => (
                <div
                  key={marker.markerId}
                  className="flex items-center gap-3 p-3 rounded-xl border border-white/5 cursor-pointer hover:border-cyan-400/20 transition-all"
                  style={{ background: "rgba(255,255,255,0.02)" }}
                  onClick={() => onMarkerClick?.(marker.lat, marker.lon)}
                >
                  <MapPin size={16} style={{ color: marker.color || COLORS.cyan }} />
                  <div className="flex-1 min-w-0">
                    <div className="text-white text-sm truncate">{marker.label || "Unnamed"}</div>
                    <div className="text-white/30 text-xs">
                      {marker.lat.toFixed(4)}, {marker.lon.toFixed(4)}
                    </div>
                  </div>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteMarker(marker.markerId);
                    }}
                    className="p-1 text-white/20 hover:text-red-400 transition-colors"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              ))}
              {markers.length === 0 && (
                <div className="text-center text-white/30 py-8 text-sm">
                  <MapPin size={24} className="mx-auto mb-2 opacity-30" />
                  No shared markers yet
                </div>
              )}
            </motion.div>
          )}

          {/* ─── Activity Feed Tab ─── */}
          {activeTab === "activity" && (
            <motion.div
              key="activity"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="p-4 space-y-2"
            >
              {events.map((event, i) => {
                const { time, label } = formatEvent(event);
                return (
                  <div
                    key={`${event.timestamp}-${i}`}
                    className="flex items-start gap-3 p-2.5 rounded-lg"
                    style={{ background: "rgba(255,255,255,0.02)" }}
                  >
                    <div
                      className="w-2 h-2 rounded-full mt-1.5 shrink-0"
                      style={{ background: event.userColor || COLORS.cyan }}
                    />
                    <div className="flex-1 min-w-0">
                      <span className="text-white/70 text-xs">
                        <span className="text-white/90 font-medium">
                          {event.userName || `User ${event.userId}`}
                        </span>{" "}
                        {label}
                      </span>
                    </div>
                    <span className="text-white/20 text-xs shrink-0">{time}</span>
                  </div>
                );
              })}
              {events.length === 0 && (
                <div className="text-center text-white/30 py-8 text-sm">
                  <MessageSquare size={24} className="mx-auto mb-2 opacity-30" />
                  No activity yet
                </div>
              )}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
