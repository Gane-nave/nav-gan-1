/**
 * G.A.N.E — Join Collaboration Session by Invite Link
 * =====================================================
 * Route: /collab/join/:token
 * 
 * Flow:
 * 1. Validate the invite token (public — shows session info even before login)
 * 2. If valid, show session info + "Join" button
 * 3. On click, redeem token + join session → redirect to Home with active session
 * 4. If invalid/expired, show error with option to go home
 */
import { useState, useEffect } from "react";
import { useRoute, useLocation } from "wouter";
import { trpc } from "@/lib/trpc";
import { useAuth } from "@/_core/hooks/useAuth";
import { getLoginUrl } from "@/const";
import { motion, AnimatePresence } from "framer-motion";
import {
  Users, Clock, Shield, AlertTriangle, Loader2,
  CheckCircle, ArrowRight, Sparkles, Globe
} from "lucide-react";
import { Button } from "@/components/ui/button";

const COLORS = {
  cyan: '#00e5ff',
  green: '#00ff88',
  red: '#ff3355',
  purple: '#aa66ff',
  gold: '#ffd700',
};

export default function JoinByInvite() {
  const [, params] = useRoute("/collab/join/:token");
  const [, navigate] = useLocation();
  const token = params?.token ?? "";
  const { user, isAuthenticated, loading: authLoading } = useAuth();
  const [joining, setJoining] = useState(false);
  const [joined, setJoined] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Validate invite token (public — works without auth)
  const { data: validation, isLoading: validating } = trpc.collaboration.validateInvite.useQuery(
    { token },
    { enabled: !!token, retry: false }
  );

  // Mutations
  const redeemInvite = trpc.collaboration.redeemInvite.useMutation();
  const joinSession = trpc.collaboration.joinSession.useMutation();

  const handleJoin = async () => {
    if (!isAuthenticated) {
      // Redirect to login with return path
      window.location.href = getLoginUrl(`/collab/join/${token}`);
      return;
    }

    setJoining(true);
    setError(null);

    try {
      // Redeem the invite token
      const { sessionId } = await redeemInvite.mutateAsync({ token });

      // Join the session
      await joinSession.mutateAsync({ sessionId });

      setJoined(true);

      // Redirect to home with session active after a brief success animation
      setTimeout(() => {
        navigate(`/?collab=${sessionId}`);
      }, 1500);
    } catch (err: any) {
      setError(err.message || "Failed to join session");
      setJoining(false);
    }
  };

  // Loading state
  if (authLoading || validating) {
    return (
      <div className="fixed inset-0 flex items-center justify-center" style={{ background: '#010206' }}>
        <motion.div
          className="flex flex-col items-center gap-6"
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
        >
          <motion.div
            className="w-16 h-16 rounded-2xl flex items-center justify-center"
            style={{
              background: `linear-gradient(135deg, rgba(0,229,255,0.15), rgba(170,102,255,0.15))`,
              border: '1px solid rgba(0,229,255,0.3)',
            }}
            animate={{ rotate: 360 }}
            transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
          >
            <Globe className="w-8 h-8" style={{ color: COLORS.cyan }} />
          </motion.div>
          <p className="text-white/60 text-sm tracking-widest uppercase">Validating invite...</p>
        </motion.div>
      </div>
    );
  }

  // Invalid or expired token
  if (validation && !validation.valid) {
    return (
      <div className="fixed inset-0 flex items-center justify-center" style={{ background: '#010206' }}>
        <motion.div
          className="max-w-md w-full mx-4 p-8 rounded-3xl text-center"
          style={{
            background: 'rgba(255,51,85,0.05)',
            border: '1px solid rgba(255,51,85,0.2)',
            boxShadow: '0 0 60px rgba(255,51,85,0.1)',
          }}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <div className="w-16 h-16 rounded-2xl flex items-center justify-center mx-auto mb-6"
            style={{
              background: 'rgba(255,51,85,0.1)',
              border: '1px solid rgba(255,51,85,0.3)',
            }}
          >
            <AlertTriangle className="w-8 h-8" style={{ color: COLORS.red }} />
          </div>
          <h2 className="text-xl font-bold text-white mb-3">Invalid Invite Link</h2>
          <p className="text-white/50 text-sm mb-6">
            This invite link is invalid, has expired, or has reached its usage limit.
          </p>
          <Button
            onClick={() => navigate("/")}
            className="px-6 py-2.5 rounded-xl font-medium"
            style={{
              background: `linear-gradient(135deg, ${COLORS.cyan}, ${COLORS.purple})`,
              color: '#fff',
            }}
          >
            Go to Home
          </Button>
        </motion.div>
      </div>
    );
  }

  // Successfully joined — show success animation
  if (joined) {
    return (
      <div className="fixed inset-0 flex items-center justify-center" style={{ background: '#010206' }}>
        <motion.div
          className="flex flex-col items-center gap-6"
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ type: 'spring', damping: 15 }}
        >
          <motion.div
            className="w-20 h-20 rounded-full flex items-center justify-center"
            style={{
              background: `linear-gradient(135deg, ${COLORS.green}20, ${COLORS.cyan}20)`,
              border: `2px solid ${COLORS.green}`,
              boxShadow: `0 0 40px ${COLORS.green}40`,
            }}
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ delay: 0.2, type: 'spring' }}
          >
            <CheckCircle className="w-10 h-10" style={{ color: COLORS.green }} />
          </motion.div>
          <h2 className="text-xl font-bold text-white">Joined Successfully!</h2>
          <p className="text-white/50 text-sm">Entering collaboration session...</p>
        </motion.div>
      </div>
    );
  }

  // Valid invite — show session info and join button
  return (
    <div className="fixed inset-0 flex items-center justify-center" style={{ background: '#010206' }}>
      {/* Background ambient glow */}
      <div className="absolute inset-0" style={{
        background: `radial-gradient(circle at 50% 30%, rgba(0,229,255,0.06) 0%, transparent 50%),
                     radial-gradient(circle at 30% 70%, rgba(170,102,255,0.04) 0%, transparent 40%)`,
      }} />

      <motion.div
        className="relative max-w-md w-full mx-4 p-8 rounded-3xl"
        style={{
          background: 'rgba(0,229,255,0.03)',
          border: '1px solid rgba(0,229,255,0.15)',
          boxShadow: '0 0 80px rgba(0,229,255,0.08)',
          backdropFilter: 'blur(20px)',
        }}
        initial={{ opacity: 0, y: 30 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        {/* Header */}
        <div className="flex items-center gap-4 mb-8">
          <div className="w-14 h-14 rounded-2xl flex items-center justify-center"
            style={{
              background: `linear-gradient(135deg, ${COLORS.cyan}20, ${COLORS.purple}20)`,
              border: `1px solid ${COLORS.cyan}40`,
            }}
          >
            <Users className="w-7 h-7" style={{ color: COLORS.cyan }} />
          </div>
          <div>
            <p className="text-white/40 text-xs tracking-widest uppercase mb-1">Collaboration Invite</p>
            <h2 className="text-lg font-bold text-white">{validation?.sessionName || "Session"}</h2>
          </div>
        </div>

        {/* Session details */}
        <div className="space-y-4 mb-8">
          <div className="flex items-center gap-3 p-3 rounded-xl"
            style={{ background: 'rgba(0,229,255,0.05)', border: '1px solid rgba(0,229,255,0.1)' }}
          >
            <Sparkles className="w-5 h-5 flex-shrink-0" style={{ color: COLORS.gold }} />
            <div>
              <p className="text-white/80 text-sm font-medium">Real-Time Collaboration</p>
              <p className="text-white/40 text-xs">Share markers, annotations, and cursors on a live map</p>
            </div>
          </div>

          <div className="flex items-center gap-3 p-3 rounded-xl"
            style={{ background: 'rgba(170,102,255,0.05)', border: '1px solid rgba(170,102,255,0.1)' }}
          >
            <Shield className="w-5 h-5 flex-shrink-0" style={{ color: COLORS.purple }} />
            <div>
              <p className="text-white/80 text-sm font-medium">Secure Session</p>
              <p className="text-white/40 text-xs">End-to-end encrypted with session isolation</p>
            </div>
          </div>
        </div>

        {/* Error message */}
        <AnimatePresence>
          {error && (
            <motion.div
              className="mb-4 p-3 rounded-xl text-sm"
              style={{ background: 'rgba(255,51,85,0.1)', border: '1px solid rgba(255,51,85,0.2)', color: COLORS.red }}
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
            >
              {error}
            </motion.div>
          )}
        </AnimatePresence>

        {/* Join button */}
        <Button
          onClick={handleJoin}
          disabled={joining}
          className="w-full py-3 rounded-xl font-bold text-base flex items-center justify-center gap-2 transition-all"
          style={{
            background: joining
              ? 'rgba(0,229,255,0.1)'
              : `linear-gradient(135deg, ${COLORS.cyan}, ${COLORS.purple})`,
            color: '#fff',
            border: 'none',
            boxShadow: joining ? 'none' : `0 0 30px rgba(0,229,255,0.3)`,
          }}
        >
          {joining ? (
            <>
              <Loader2 className="w-5 h-5 animate-spin" />
              Joining...
            </>
          ) : !isAuthenticated ? (
            <>
              Sign In & Join Session
              <ArrowRight className="w-5 h-5" />
            </>
          ) : (
            <>
              Join Session
              <ArrowRight className="w-5 h-5" />
            </>
          )}
        </Button>

        {!isAuthenticated && (
          <p className="text-center text-white/30 text-xs mt-3">
            You'll be redirected to sign in first
          </p>
        )}
      </motion.div>
    </div>
  );
}
