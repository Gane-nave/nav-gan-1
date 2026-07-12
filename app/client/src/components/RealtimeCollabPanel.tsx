/**
 * G.A.N.E NAV — Real-Time Collaboration Panel
 * =============================================
 * Live position sharing, fleet tracking, team coordination,
 * geofences, SOS beacon, and breadcrumb trails.
 */
import { useState, useEffect, useMemo, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Users, Plus, X, MapPin, Wifi, WifiOff, Copy, Check,
  Radio, Shield, AlertTriangle, Navigation, Battery,
  Signal, Clock, Route, Eye, EyeOff, Bell, Zap,
  UserPlus, LogOut, Share2, Target, Hexagon, Activity
} from 'lucide-react';
import { useLanguage } from '@/contexts/LanguageContext';
import {
  getRealtimeCollabEngine,
  type RealtimeCollabState, type TeamMember, type ConnectionTier
} from '@/engine/realtimeCollabEngine';

const TIER_INFO: Record<ConnectionTier, { label: string; color: string; icon: typeof Wifi }> = {
  webrtc: { label: 'P2P Direct', color: '#00cc66', icon: Zap },
  sse: { label: 'Server Relay', color: '#4488ff', icon: Wifi },
  polling: { label: 'HTTP Polling', color: '#ff9900', icon: Activity },
  offline: { label: 'Offline', color: '#ff3355', icon: WifiOff },
};

function MemberCard({ member, isMe }: { member: TeamMember; isMe: boolean }) {
  const statusColors: Record<string, string> = {
    active: '#00cc66', idle: '#ff9900', sos: '#ff3355', offline: '#666', privacy: '#aa66ff',
  };

  return (
    <motion.div
      className="rounded-xl p-3 mb-2"
      style={{
        background: isMe ? 'rgba(68,136,255,0.06)' : 'rgba(255,255,255,0.7)',
        border: `1px solid ${isMe ? 'rgba(68,136,255,0.2)' : member.status === 'sos' ? 'rgba(255,51,85,0.4)' : 'rgba(0,0,0,0.06)'}`,
        boxShadow: member.status === 'sos' ? '0 0 15px rgba(255,51,85,0.2)' : undefined,
      }}
      layout
      animate={member.status === 'sos' ? { scale: [1, 1.01, 1] } : {}}
      transition={member.status === 'sos' ? { repeat: Infinity, duration: 1 } : {}}
    >
      <div className="flex items-center gap-3">
        {/* Avatar */}
        <div className="relative">
          <div
            className="w-9 h-9 rounded-full flex items-center justify-center text-xs font-bold text-white"
            style={{ background: member.color }}
          >
            {member.name.charAt(0).toUpperCase()}
          </div>
          <div
            className="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-white"
            style={{ background: statusColors[member.status] }}
          />
        </div>

        {/* Info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-1.5">
            <span className="text-xs font-semibold truncate" style={{ color: '#1a1a2e' }}>
              {member.name} {isMe && '(You)'}
            </span>
            {member.role === 'leader' && (
              <Shield size={10} style={{ color: '#ffd700' }} />
            )}
            {member.status === 'sos' && (
              <span className="text-[9px] font-bold px-1.5 py-0.5 rounded-full bg-red-500 text-white animate-pulse">
                SOS
              </span>
            )}
          </div>
          <div className="flex items-center gap-2 mt-0.5">
            <span className="text-[10px]" style={{ color: 'rgba(0,0,0,0.4)' }}>
              {member.position.speed.toFixed(0)} km/h
            </span>
            <span className="text-[10px]" style={{ color: 'rgba(0,0,0,0.3)' }}>·</span>
            <span className="text-[10px]" style={{ color: 'rgba(0,0,0,0.4)' }}>
              {member.position.accuracy.toFixed(0)}m acc
            </span>
            {member.eta && (
              <>
                <span className="text-[10px]" style={{ color: 'rgba(0,0,0,0.3)' }}>·</span>
                <span className="text-[10px]" style={{ color: '#4488ff' }}>
                  ETA {member.eta.minutes.toFixed(0)}min
                </span>
              </>
            )}
          </div>
        </div>

        {/* Status indicators */}
        <div className="flex flex-col items-end gap-1">
          <div className="flex items-center gap-1">
            <Battery size={10} style={{ color: member.battery < 20 ? '#ff3355' : 'rgba(0,0,0,0.3)' }} />
            <span className="text-[10px]" style={{ color: member.battery < 20 ? '#ff3355' : 'rgba(0,0,0,0.4)' }}>
              {member.battery.toFixed(0)}%
            </span>
          </div>
          <div className="flex items-center gap-0.5">
            {Array.from({ length: 5 }).map((_, i) => (
              <div
                key={i}
                className="rounded-sm"
                style={{
                  width: 2, height: 4 + i * 2,
                  background: i < member.signal ? '#00cc66' : 'rgba(0,0,0,0.1)',
                }}
              />
            ))}
          </div>
        </div>
      </div>

      {/* Vehicle info */}
      {member.vehicle && (
        <div className="mt-1.5 text-[10px] flex items-center gap-1.5" style={{ color: 'rgba(0,0,0,0.4)' }}>
          <Navigation size={9} />
          {member.vehicle.type} · {member.vehicle.plate}
        </div>
      )}
    </motion.div>
  );
}

export default function RealtimeCollabPanel({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const engine = useMemo(() => getRealtimeCollabEngine(), []);
  const [state, setState] = useState<RealtimeCollabState>(engine.getState());
  const [activeTab, setActiveTab] = useState<'team' | 'geofences' | 'alerts'>('team');
  const [showCreate, setShowCreate] = useState(false);
  const [showJoin, setShowJoin] = useState(false);
  const [sessionName, setSessionName] = useState('');
  const [joinCode, setJoinCode] = useState('');
  const [joinName, setJoinName] = useState('');
  const [copiedCode, setCopiedCode] = useState(false);

  useEffect(() => {
    const unsub = engine.subscribe(setState);
    return () => { unsub(); };
  }, [engine]);

  const handleCreate = useCallback(async () => {
    if (!sessionName.trim()) return;
    await engine.createSession(sessionName);
    setShowCreate(false);
    setSessionName('');
  }, [engine, sessionName]);

  const handleJoin = useCallback(async () => {
    if (!joinCode.trim() || !joinName.trim()) return;
    await engine.joinSession(joinCode, joinName);
    setShowJoin(false);
    setJoinCode('');
    setJoinName('');
  }, [engine, joinCode, joinName]);

  const copyCode = useCallback(() => {
    if (state.activeSession) {
      navigator.clipboard.writeText(state.activeSession.code).catch(() => {});
      setCopiedCode(true);
      setTimeout(() => setCopiedCode(false), 2000);
    }
  }, [state.activeSession]);

  const tierInfo = TIER_INFO[state.connectionTier];
  const TierIcon = tierInfo.icon;

  const tabs = [
    { id: 'team' as const, label: 'Team', icon: Users, count: state.activeSession?.members.length ?? 0 },
    { id: 'geofences' as const, label: 'Zones', icon: Target, count: state.activeSession?.geofences.length ?? 0 },
    { id: 'alerts' as const, label: 'Alerts', icon: Bell, count: state.activeSession?.alerts.filter(a => !a.acknowledged).length ?? 0 },
  ];

  return (
    <motion.div
      className="h-full flex flex-col"
      style={{ direction: dir }}
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
    >
      {/* Header */}
      <div className="px-5 pt-5 pb-3">
        <div className="flex items-center gap-3 mb-3">
          <div className="w-10 h-10 rounded-xl flex items-center justify-center"
            style={{ background: 'linear-gradient(135deg, #4488ff, #aa66ff)' }}>
            <Users size={20} color="#fff" />
          </div>
          <div>
            <h2 className="text-base font-bold" style={{ color: '#1a1a2e' }}>
              {state.activeSession ? state.activeSession.name : 'Real-Time Collab'}
            </h2>
            <div className="flex items-center gap-2 text-xs" style={{ color: 'rgba(0,0,0,0.5)' }}>
              <TierIcon size={11} style={{ color: tierInfo.color }} />
              <span style={{ color: tierInfo.color }}>{tierInfo.label}</span>
              {state.isConnected && (
                <>
                  <span>·</span>
                  <span>{state.latencyMs.toFixed(0)}ms</span>
                  <span>·</span>
                  <span>{state.peerCount} peers</span>
                </>
              )}
            </div>
          </div>
        </div>

        {/* Session code */}
        {state.activeSession && (
          <div className="flex items-center gap-2 mb-3">
            <div
              className="flex-1 flex items-center gap-2 px-3 py-2 rounded-lg"
              style={{ background: 'rgba(0,0,0,0.03)', border: '1px solid rgba(0,0,0,0.06)' }}
            >
              <span className="text-[10px] uppercase" style={{ color: 'rgba(0,0,0,0.4)' }}>Code:</span>
              <span className="text-sm font-mono font-bold tracking-wider" style={{ color: '#1a1a2e' }}>
                {state.activeSession.code}
              </span>
            </div>
            <button
              onClick={copyCode}
              className="p-2 rounded-lg transition-all hover:scale-105"
              style={{ background: copiedCode ? 'rgba(0,204,102,0.1)' : 'rgba(0,0,0,0.04)' }}
            >
              {copiedCode ? <Check size={14} style={{ color: '#00cc66' }} /> : <Copy size={14} style={{ color: 'rgba(0,0,0,0.4)' }} />}
            </button>
          </div>
        )}

        {/* SOS + Share buttons */}
        {state.activeSession && (
          <div className="flex gap-2 mb-3">
            <button
              onClick={() => state.isSharingPosition ? engine.stopSharingPosition() : engine.startSharingPosition()}
              className="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-xs font-medium transition-all hover:scale-[1.02]"
              style={{
                background: state.isSharingPosition ? 'rgba(0,204,102,0.1)' : 'rgba(0,0,0,0.04)',
                color: state.isSharingPosition ? '#00cc66' : 'rgba(0,0,0,0.5)',
              }}
            >
              {state.isSharingPosition ? <Eye size={12} /> : <EyeOff size={12} />}
              {state.isSharingPosition ? 'Sharing' : 'Share'}
            </button>
            <button
              onClick={() => state.sosActive ? engine.deactivateSOS() : engine.activateSOS()}
              className="flex items-center justify-center gap-1.5 px-4 py-2 rounded-lg text-xs font-bold transition-all hover:scale-[1.02]"
              style={{
                background: state.sosActive ? '#ff3355' : 'rgba(255,51,85,0.08)',
                color: state.sosActive ? '#fff' : '#ff3355',
              }}
            >
              <AlertTriangle size={12} />
              SOS
            </button>
            <button
              onClick={() => engine.leaveSession()}
              className="flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium transition-all hover:scale-[1.02]"
              style={{ background: 'rgba(0,0,0,0.04)', color: 'rgba(0,0,0,0.5)' }}
            >
              <LogOut size={12} />
            </button>
          </div>
        )}
      </div>

      {/* No session — Create/Join */}
      {!state.activeSession && (
        <div className="px-5 flex-1">
          <div className="flex gap-2 mb-4">
            <button
              onClick={() => { setShowCreate(true); setShowJoin(false); }}
              className="flex-1 flex items-center justify-center gap-2 py-3 rounded-xl text-xs font-medium transition-all hover:scale-[1.02]"
              style={{ background: '#4488ff', color: '#fff' }}
            >
              <Plus size={14} /> Create Session
            </button>
            <button
              onClick={() => { setShowJoin(true); setShowCreate(false); }}
              className="flex-1 flex items-center justify-center gap-2 py-3 rounded-xl text-xs font-medium transition-all hover:scale-[1.02]"
              style={{ background: 'rgba(68,136,255,0.08)', color: '#4488ff' }}
            >
              <UserPlus size={14} /> Join Session
            </button>
          </div>

          <AnimatePresence>
            {showCreate && (
              <motion.div
                className="rounded-xl p-4"
                style={{ background: 'rgba(68,136,255,0.04)', border: '1px solid rgba(68,136,255,0.15)' }}
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
              >
                <input
                  type="text"
                  placeholder="Session name..."
                  value={sessionName}
                  onChange={e => setSessionName(e.target.value)}
                  className="w-full px-3 py-2 rounded-lg text-xs mb-3"
                  style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                />
                <button
                  onClick={handleCreate}
                  className="w-full py-2 rounded-lg text-xs font-medium"
                  style={{ background: '#4488ff', color: '#fff' }}
                >
                  Create & Start Sharing
                </button>
              </motion.div>
            )}
            {showJoin && (
              <motion.div
                className="rounded-xl p-4"
                style={{ background: 'rgba(68,136,255,0.04)', border: '1px solid rgba(68,136,255,0.15)' }}
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
              >
                <input
                  type="text"
                  placeholder="Session code..."
                  value={joinCode}
                  onChange={e => setJoinCode(e.target.value.toUpperCase())}
                  className="w-full px-3 py-2 rounded-lg text-xs mb-2 font-mono tracking-wider"
                  style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                />
                <input
                  type="text"
                  placeholder="Your name..."
                  value={joinName}
                  onChange={e => setJoinName(e.target.value)}
                  className="w-full px-3 py-2 rounded-lg text-xs mb-3"
                  style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                />
                <button
                  onClick={handleJoin}
                  className="w-full py-2 rounded-lg text-xs font-medium"
                  style={{ background: '#4488ff', color: '#fff' }}
                >
                  Join Session
                </button>
              </motion.div>
            )}
          </AnimatePresence>

          {/* Empty state */}
          {!showCreate && !showJoin && (
            <div className="text-center py-12">
              <Users size={40} className="mx-auto mb-3" style={{ color: 'rgba(0,0,0,0.1)' }} />
              <div className="text-sm font-medium" style={{ color: 'rgba(0,0,0,0.4)' }}>
                No Active Session
              </div>
              <div className="text-xs mt-1" style={{ color: 'rgba(0,0,0,0.3)' }}>
                Create or join a session to start sharing positions in real-time
              </div>
            </div>
          )}
        </div>
      )}

      {/* Active session content */}
      {state.activeSession && (
        <>
          {/* Tabs */}
          <div className="flex px-5 gap-1 mb-3">
            {tabs.map(tab => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all"
                style={{
                  background: activeTab === tab.id ? '#4488ff' : 'rgba(0,0,0,0.04)',
                  color: activeTab === tab.id ? '#fff' : 'rgba(0,0,0,0.5)',
                }}
              >
                <tab.icon size={12} />
                {tab.label}
                {tab.count > 0 && (
                  <span
                    className="text-[9px] px-1.5 py-0.5 rounded-full"
                    style={{
                      background: activeTab === tab.id ? 'rgba(255,255,255,0.2)' : 'rgba(0,0,0,0.08)',
                      color: activeTab === tab.id ? '#fff' : 'rgba(0,0,0,0.4)',
                    }}
                  >
                    {tab.count}
                  </span>
                )}
              </button>
            ))}
          </div>

          {/* Tab content */}
          <div className="flex-1 overflow-y-auto px-5 pb-5">
            <AnimatePresence mode="wait">
              {activeTab === 'team' && (
                <motion.div key="team" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                  {state.activeSession.members.map(member => (
                    <MemberCard
                      key={member.id}
                      member={member}
                      isMe={member.id === state.myId}
                    />
                  ))}

                  {/* Stats */}
                  <div className="mt-3 grid grid-cols-3 gap-2">
                    {[
                      { label: 'Sent', value: state.stats.messagesSent.toLocaleString() },
                      { label: 'Received', value: state.stats.messagesReceived.toLocaleString() },
                      { label: 'Uptime', value: `${Math.floor(state.stats.uptime / 60)}m` },
                    ].map(stat => (
                      <div
                        key={stat.label}
                        className="rounded-lg p-2 text-center"
                        style={{ background: 'rgba(0,0,0,0.03)' }}
                      >
                        <div className="text-sm font-bold" style={{ color: '#1a1a2e' }}>{stat.value}</div>
                        <div className="text-[9px]" style={{ color: 'rgba(0,0,0,0.4)' }}>{stat.label}</div>
                      </div>
                    ))}
                  </div>
                </motion.div>
              )}

              {activeTab === 'geofences' && (
                <motion.div key="geofences" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                  <button
                    onClick={() => engine.addGeofence({
                      name: `Zone ${(state.activeSession?.geofences.length ?? 0) + 1}`,
                      type: 'circle',
                      center: { lat: 32.085, lon: 34.782 },
                      radius: 500,
                      alertOnEnter: true,
                      alertOnExit: true,
                      active: true,
                      color: '#4488ff',
                    })}
                    className="w-full flex items-center justify-center gap-2 py-2.5 rounded-xl mb-3 text-xs font-medium"
                    style={{ background: 'rgba(68,136,255,0.08)', color: '#4488ff', border: '1px dashed rgba(68,136,255,0.3)' }}
                  >
                    <Plus size={14} /> Add Geofence
                  </button>

                  {state.activeSession.geofences.length === 0 ? (
                    <div className="text-center py-8">
                      <Target size={32} className="mx-auto mb-2" style={{ color: 'rgba(0,0,0,0.15)' }} />
                      <div className="text-xs" style={{ color: 'rgba(0,0,0,0.4)' }}>No geofences defined</div>
                    </div>
                  ) : (
                    state.activeSession.geofences.map(fence => (
                      <div
                        key={fence.id}
                        className="rounded-xl p-3 mb-2 flex items-center justify-between"
                        style={{ background: 'rgba(255,255,255,0.7)', border: '1px solid rgba(0,0,0,0.06)' }}
                      >
                        <div>
                          <div className="flex items-center gap-2">
                            <div className="w-2.5 h-2.5 rounded-full" style={{ background: fence.color }} />
                            <span className="text-xs font-medium" style={{ color: '#1a1a2e' }}>{fence.name}</span>
                          </div>
                          <div className="text-[10px] mt-0.5" style={{ color: 'rgba(0,0,0,0.4)' }}>
                            {fence.radius}m radius · {fence.alertOnEnter ? 'Enter' : ''}{fence.alertOnEnter && fence.alertOnExit ? ' + ' : ''}{fence.alertOnExit ? 'Exit' : ''} alerts
                          </div>
                        </div>
                        <button
                          onClick={() => engine.removeGeofence(fence.id)}
                          className="p-1.5 rounded-lg hover:bg-red-50"
                        >
                          <X size={12} style={{ color: '#ff3355' }} />
                        </button>
                      </div>
                    ))
                  )}
                </motion.div>
              )}

              {activeTab === 'alerts' && (
                <motion.div key="alerts" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                  {state.activeSession.alerts.length === 0 ? (
                    <div className="text-center py-8">
                      <Bell size={32} className="mx-auto mb-2" style={{ color: 'rgba(0,0,0,0.15)' }} />
                      <div className="text-xs" style={{ color: 'rgba(0,0,0,0.4)' }}>No alerts yet</div>
                    </div>
                  ) : (
                    state.activeSession.alerts.slice(0, 30).map(alert => {
                      const priorityColors = { critical: '#ff3355', warning: '#ff9900', info: '#4488ff' };
                      return (
                        <div
                          key={alert.id}
                          className="rounded-xl p-3 mb-2"
                          style={{
                            background: alert.acknowledged ? 'rgba(0,0,0,0.02)' : 'rgba(255,255,255,0.7)',
                            border: `1px solid ${alert.acknowledged ? 'rgba(0,0,0,0.04)' : priorityColors[alert.priority] + '20'}`,
                            opacity: alert.acknowledged ? 0.6 : 1,
                          }}
                          onClick={() => engine.acknowledgeAlert(alert.id)}
                        >
                          <div className="flex items-start gap-2">
                            <div
                              className="w-1.5 h-1.5 rounded-full mt-1.5 shrink-0"
                              style={{ background: priorityColors[alert.priority] }}
                            />
                            <div className="flex-1 min-w-0">
                              <div className="text-xs" style={{ color: '#1a1a2e' }}>{alert.message}</div>
                              <div className="text-[10px] mt-0.5" style={{ color: 'rgba(0,0,0,0.3)' }}>
                                {new Date(alert.timestamp).toLocaleTimeString()}
                              </div>
                            </div>
                          </div>
                        </div>
                      );
                    })
                  )}
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </>
      )}
    </motion.div>
  );
}
