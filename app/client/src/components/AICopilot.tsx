/**
 * G.A.N.E — AI Copilot v3.0
 * Ambient intelligence assistant with contextual awareness, gesture hints, 
 * proactive suggestions, and natural language interaction
 */
import { useState, useEffect, useRef, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Sparkles, X, Send, Mic, MicOff, ChevronDown,
  Navigation, Cloud, AlertTriangle, Zap, TrendingUp,
  Clock, MapPin, Compass, Shield, Fuel, Coffee,
  Brain, Eye, Lightbulb, MessageCircle
} from 'lucide-react';
import { useRealDataContext } from '@/contexts/RealDataContext';
import { useLanguage } from "@/contexts/LanguageContext";

// ═══════════════════════════════════════════════════
// AI Insight Engine — generates contextual suggestions
// ═══════════════════════════════════════════════════
interface AISuggestion {
  id: string;
  type: 'route' | 'weather' | 'safety' | 'fuel' | 'time' | 'discovery' | 'alert';
  icon: any;
  title: string;
  titleHe: string;
  description: string;
  descriptionHe: string;
  color: string;
  priority: number;
  action?: string;
  timestamp: number;
}

function generateInsights(weather: any, earthquakes: any[], location: any): AISuggestion[] {
  const insights: AISuggestion[] = [];
  const now = Date.now();
  const hour = new Date().getHours();

  // Weather-based insights
  if (weather) {
    if (weather.temperature > 35) {
      insights.push({
        id: 'heat-warning', type: 'weather', icon: AlertTriangle,
        title: 'Extreme Heat Alert', titleHe: 'התראת חום קיצוני',
        description: `${Math.round(weather.temperature)}°C — Stay hydrated, AC recommended`,
        descriptionHe: `${Math.round(weather.temperature)}°C — שתה מים, מומלץ מזגן`,
        color: '#ff4466', priority: 9, timestamp: now });
    }
    if (weather.windSpeed > 40) {
      insights.push({
        id: 'wind-warning', type: 'safety', icon: Shield,
        title: 'High Wind Warning', titleHe: 'אזהרת רוחות חזקות',
        description: `Wind at ${Math.round(weather.windSpeed)} km/h — Drive carefully`,
        descriptionHe: `רוח ${Math.round(weather.windSpeed)} קמ"ש — נהג בזהירות`,
        color: '#ffaa00', priority: 8, timestamp: now });
    }
    if (weather.visibility < 2) {
      insights.push({
        id: 'visibility', type: 'safety', icon: Eye,
        title: 'Low Visibility', titleHe: 'ראות מוגבלת',
        description: `Visibility ${weather.visibility.toFixed(1)} km — Use fog lights`,
        descriptionHe: `ראות ${weather.visibility.toFixed(1)} ק"מ — הדלק אורות ערפל`,
        color: '#ff8800', priority: 8, timestamp: now });
    }
    if (weather.weatherDescription?.toLowerCase().includes('rain')) {
      insights.push({
        id: 'rain', type: 'weather', icon: Cloud,
        title: 'Rain Detected', titleHe: 'גשם מזוהה',
        description: 'Roads may be slippery — increase following distance',
        descriptionHe: 'כבישים עלולים להיות חלקלקים — הגדל מרחק',
        color: '#2563EB', priority: 7, timestamp: now });
    }
  }

  // Earthquake insights
  if (earthquakes && earthquakes.length > 0) {
    const recent = earthquakes.filter(e => now - e.time < 3600000);
    if (recent.length > 0) {
      const strongest = recent.reduce((a, b) => a.magnitude > b.magnitude ? a : b);
      if (strongest.magnitude >= 3) {
        insights.push({
          id: 'earthquake', type: 'alert', icon: AlertTriangle,
          title: 'Seismic Activity', titleHe: 'פעילות סיסמית',
          description: `M${strongest.magnitude.toFixed(1)} earthquake detected nearby`,
          descriptionHe: `רעידת אדמה M${strongest.magnitude.toFixed(1)} זוהתה בקרבת מקום`,
          color: '#ff4466', priority: 10, timestamp: now });
      }
    }
  }

  // Time-based insights
  if (hour >= 7 && hour <= 9) {
    insights.push({
      id: 'morning-commute', type: 'time', icon: Clock,
      title: 'Morning Rush', titleHe: 'שעת שיא בוקר',
      description: 'Consider alternative routes to avoid congestion',
      descriptionHe: 'שקול מסלולים חלופיים להימנע מפקקים',
      color: '#ffaa00', priority: 5, timestamp: now });
  }
  if (hour >= 16 && hour <= 19) {
    insights.push({
      id: 'evening-commute', type: 'time', icon: Clock,
      title: 'Evening Rush', titleHe: 'שעת שיא ערב',
      description: 'Heavy traffic expected — plan extra time',
      descriptionHe: 'צפוי עומס תנועה — תכנן זמן נוסף',
      color: '#ff8800', priority: 5, timestamp: now });
  }

  // Discovery insights
  insights.push({
    id: 'fuel-tip', type: 'fuel', icon: Fuel,
    title: 'Fuel Optimization', titleHe: 'אופטימיזציית דלק',
    description: 'Cheapest fuel station 2.3 km ahead on your route',
    descriptionHe: 'תחנת הדלק הזולה ביותר 2.3 ק"מ לפניך',
    color: '#00e88f', priority: 3, timestamp: now });

  if (hour >= 10 && hour <= 14) {
    insights.push({
      id: 'coffee-break', type: 'discovery', icon: Coffee,
      title: 'Coffee Break?', titleHe: 'הפסקת קפה?',
      description: 'Highly rated cafe 500m from your location',
      descriptionHe: 'בית קפה מדורג גבוה 500 מטר ממיקומך',
      color: '#ff8800', priority: 2, timestamp: now });
  }

  insights.push({
    id: 'eco-route', type: 'route', icon: TrendingUp,
    title: 'Eco Route Available', titleHe: 'מסלול ירוק זמין',
    description: 'Save 15% fuel with a 3 min longer route',
    descriptionHe: 'חסוך 15% דלק עם מסלול ארוך ב-3 דקות',
    color: '#00e88f', priority: 4, timestamp: now });

  return insights.sort((a, b) => b.priority - a.priority);
}

// ═══════════════════════════════════════════════════
// Floating AI Orb — always-visible entry point
// ═══════════════════════════════════════════════════
export function AIOrb({ onClick, hasAlerts }: { onClick: () => void; hasAlerts: boolean }) {
  return (
    <motion.button
      onClick={onClick}
      className="relative w-16 h-16 rounded-full flex items-center justify-center cursor-pointer group"
      style={{
        background: 'radial-gradient(circle, rgba(0,212,255,0.2) 0%, rgba(0,212,255,0.05) 60%, transparent 100%)',
        boxShadow: '0 0 30px rgba(0,212,255,0.2), 0 0 60px rgba(0,212,255,0.08)' }}
      whileHover={{ scale: 1.15 }}
      whileTap={{ scale: 0.85 }}
      animate={{
        boxShadow: hasAlerts
          ? ['0 0 30px rgba(0,212,255,0.2), 0 0 60px rgba(0,212,255,0.08)', '0 0 50px rgba(0,212,255,0.4), 0 0 80px rgba(0,212,255,0.15)', '0 0 30px rgba(0,212,255,0.2), 0 0 60px rgba(0,212,255,0.08)']
          : '0 0 30px rgba(0,212,255,0.2), 0 0 60px rgba(0,212,255,0.08)' }}
      transition={hasAlerts ? { duration: 2.5, repeat: Infinity } : {}}
    >
      {/* Inner core glow */}
      <motion.div
        className="absolute w-8 h-8 rounded-full"
        style={{ background: 'radial-gradient(circle, rgba(0,212,255,0.4), transparent 70%)' }}
        animate={{ scale: [1, 1.3, 1], opacity: [0.6, 1, 0.6] }}
        transition={{ duration: 3, repeat: Infinity }}
      />

      <Brain className="w-6 h-6 relative z-10" style={{ color: '#2563EB', filter: 'drop-shadow(0 0 6px rgba(37,99,235,0.5))' }} />

      {/* Orbiting ring 1 — fast */}
      <motion.div
        className="absolute inset-[-4px] rounded-full pointer-events-none"
        style={{ border: '1.5px solid rgba(0,212,255,0.25)' }}
        animate={{ rotate: 360 }}
        transition={{ duration: 6, repeat: Infinity, ease: 'linear' }}
      />

      {/* Orbiting ring 2 — slow, opposite */}
      <motion.div
        className="absolute inset-[-10px] rounded-full pointer-events-none"
        style={{ border: '1px solid rgba(0,212,255,0.12)' }}
        animate={{ rotate: -360 }}
        transition={{ duration: 12, repeat: Infinity, ease: 'linear' }}
      />

      {/* Orbiting ring 3 — outer pulse */}
      <motion.div
        className="absolute inset-[-16px] rounded-full pointer-events-none"
        style={{ border: '1px solid rgba(0,212,255,0.06)' }}
        animate={{ scale: [1, 1.08, 1], opacity: [0.3, 0.6, 0.3] }}
        transition={{ duration: 4, repeat: Infinity }}
      />

      {/* Orbiting particle dots */}
      {[0, 120, 240].map((angle, i) => (
        <motion.div
          key={i}
          className="absolute w-1.5 h-1.5 rounded-full pointer-events-none"
          style={{
            background: i === 0 ? '#2563EB' : i === 1 ? '#7C3AED' : '#16A34A',
            boxShadow: `0 0 6px ${i === 0 ? '#2563EB' : i === 1 ? '#7C3AED' : '#16A34A'}`,
            top: '50%', left: '50%',
            transformOrigin: '0 0' }}
          animate={{ rotate: 360 }}
          transition={{ duration: 4 + i * 2, repeat: Infinity, ease: 'linear' }}
          initial={{ x: Math.cos(angle * Math.PI / 180) * 28, y: Math.sin(angle * Math.PI / 180) * 28 }}
        />
      ))}

      {/* Alert badge */}
      {hasAlerts && (
        <motion.div
          className="absolute -top-1 -right-1 w-4 h-4 rounded-full flex items-center justify-center"
          style={{ background: '#ff4466', boxShadow: '0 0 12px #ff446680', fontSize: 9, color: 'white', fontWeight: 700 }}
          animate={{ scale: [1, 1.2, 1] }}
          transition={{ duration: 1.5, repeat: Infinity }}
        >
          !
        </motion.div>
      )}

      {/* Hover label */}
      <div className="absolute -bottom-7 left-1/2 -translate-x-1/2 opacity-0 group-hover:opacity-100 transition-opacity duration-300 whitespace-nowrap text-[10px] font-bold tracking-wider" style={{ color: '#2563EB' }}>
        AI COPILOT
      </div>
    </motion.button>
  );
}

// ═══════════════════════════════════════════════════
// AI Chat Panel — full conversational interface
// ═══════════════════════════════════════════════════
interface ChatMessage {
  id: string;
  role: 'user' | 'ai';
  text: string;
  timestamp: number;
}

export function AIChatPanel({ isOpen, onClose }: { isOpen: boolean; onClose: () => void }) {
  const { t, dir, lang } = useLanguage();
  const { weather, earthquakes, location } = useRealDataContext();
  const [insights, setInsights] = useState<AISuggestion[]>([]);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [isListening, setIsListening] = useState(false);
  const [activeTab, setActiveTab] = useState<'insights' | 'chat'>('insights');
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const newInsights = generateInsights(weather, earthquakes || [], location);
    setInsights(newInsights);
  }, [weather, earthquakes, location]);

  const sendMessage = useCallback(() => {
    if (!input.trim()) return;
    const userMsg: ChatMessage = { id: `u-${Date.now()}`, role: 'user', text: input, timestamp: Date.now() };
    setMessages(prev => [...prev, userMsg]);
    setInput('');

    // AI response simulation
    setTimeout(() => {
      const responses = [
        { trigger: /מסלול|route|דרך/i, text: 'מחשב מסלול אופטימלי... המסלול המהיר ביותר כרגע הוא דרך כביש 2 דרום. זמן משוער: 23 דקות.' },
        { trigger: /מזג|weather|גשם|rain/i, text: weather ? `מזג האוויר הנוכחי: ${Math.round(weather.temperature)}°C, ${weather.weatherDescription}. לחות ${weather.humidity}%. רוח ${Math.round(weather.windSpeed)} קמ"ש.` : 'טוען נתוני מזג אוויר...' },
        { trigger: /חניה|parking/i, text: 'מצאתי 3 חניונים פנויים בקרבת מקום. הקרוב ביותר: חניון דיזנגוף, 340 מטר, 12 מקומות פנויים.' },
        { trigger: /דלק|fuel|בנזין/i, text: 'תחנת הדלק הזולה ביותר: סונול כביש 4, 2.3 ק"מ. מחיר: 6.82 ₪/ליטר. חיסכון של 0.15 ₪ לעומת הממוצע.' },
        { trigger: /תנועה|traffic|פקק/i, text: 'מצב התנועה: איילון צפון — עומס בינוני (38 קמ"ש). כביש בגין — עומס כבד (15 קמ"ש). כביש 2 — חופשי (92 קמ"ש).' },
      ];
      const match = responses.find(r => r.trigger.test(input));
      const aiText = match?.text || 'אני G.A.N.E AI. אני יכול לעזור עם ניווט, מזג אוויר, תנועה, חניה, דלק, ועוד. מה תרצה לדעת?';
      const aiMsg: ChatMessage = { id: `a-${Date.now()}`, role: 'ai', text: aiText, timestamp: Date.now() };
      setMessages(prev => [...prev, aiMsg]);
    }, 800);
  }, [input, weather]);

  useEffect(() => {
    if (scrollRef.current) scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
  }, [messages]);

  if (!isOpen) return null;

  return (
    <motion.div
      initial={{ opacity: 0, y: 20, scale: 0.95 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: 20, scale: 0.95 }}
      transition={{ type: 'spring', damping: 24, stiffness: 300 }}
      className="fixed bottom-24 right-4 z-50 w-[340px] max-h-[520px] rounded-2xl overflow-hidden flex flex-col"
      style={{
        background: 'rgba(6,8,14,0.96)',
        border: '1px solid rgba(0,212,255,0.12)',
        boxShadow: '0 8px 40px rgba(249,250,251,0.8), 0 0 30px rgba(0,212,255,0.05)' }}
    >
      {/* ── Header ── */}
      <div className="flex items-center justify-between px-4 py-3"
        style={{ borderBottom: '1px solid rgba(229,231,235,0.4)' }}>
        <div className="flex items-center gap-2.5">
          <motion.div
            className="w-8 h-8 rounded-lg flex items-center justify-center relative"
            style={{ background: 'rgba(0,212,255,0.1)' }}
          >
            <Brain className="w-4 h-4" style={{ color: '#2563EB' }} />
            <motion.div
              className="absolute inset-0 rounded-lg"
              style={{ border: '1px solid rgba(0,212,255,0.3)' }}
              animate={{ opacity: [0.3, 0.7, 0.3] }}
              transition={{ duration: 2, repeat: Infinity }}
            />
          </motion.div>
          <div>
            <div className="text-xs font-bold" style={{ fontFamily: 'Syne, sans-serif', color: '#2563EB' }}>G.A.N.E AI</div>
            <div className="text-[9px] text-white/20">Ambient Intelligence</div>
          </div>
        </div>
        <motion.button
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.85 }}
          onClick={onClose}
          className="w-7 h-7 rounded-lg flex items-center justify-center cursor-pointer"
          style={{ background: 'rgba(229,231,235,0.4)' }}
        >
          <X className="w-3.5 h-3.5 text-white/30" />
        </motion.button>
      </div>

      {/* ── Tabs ── */}
      <div className="flex px-3 pt-2 gap-1">
        {[
          { id: 'insights' as const, label: t('ai.insights'), icon: Lightbulb },
          { id: 'chat' as const, label: t('ai.chat'), icon: MessageCircle },
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-[10px] font-medium transition-all cursor-pointer"
            style={{
              background: activeTab === tab.id ? 'rgba(0,212,255,0.08)' : 'transparent',
              color: activeTab === tab.id ? '#2563EB' : 'rgba(156,163,175,0.8)',
              border: activeTab === tab.id ? '1px solid rgba(0,212,255,0.15)' : '1px solid transparent' }}
          >
            <tab.icon className="w-3 h-3" />
            {tab.label}
          </button>
        ))}
      </div>

      {/* ── Content ── */}
      <div ref={scrollRef} className="flex-1 overflow-y-auto px-3 py-2" style={{ scrollbarWidth: 'none' }}>
        <AnimatePresence mode="wait">
          {activeTab === 'insights' ? (
            <motion.div
              key="insights"
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: 10 }}
              className="space-y-2"
            >
              {insights.length === 0 ? (
                <div className="text-center py-8">
                  <Sparkles className="w-8 h-8 mx-auto mb-2" style={{ color: 'rgba(0,212,255,0.2)' }} />
                  <div className="text-xs text-white/20">{t('ai.noInsights')}</div>
                </div>
              ) : (
                insights.map((insight, i) => (
                  <motion.div
                    key={insight.id}
                    initial={{ opacity: 0, y: 8 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: i * 0.05 }}
                    className="rounded-xl p-3 transition-all duration-200 cursor-pointer group"
                    style={{
                      background: `${insight.color}06`,
                      border: `1px solid ${insight.color}12` }}
                    onMouseEnter={e => {
                      e.currentTarget.style.background = `${insight.color}10`;
                      e.currentTarget.style.borderColor = `${insight.color}25`;
                    }}
                    onMouseLeave={e => {
                      e.currentTarget.style.background = `${insight.color}06`;
                      e.currentTarget.style.borderColor = `${insight.color}12`;
                    }}
                  >
                    <div className="flex items-start gap-2.5" style={{ direction: dir }}>
                      <div className="w-7 h-7 rounded-lg flex items-center justify-center flex-shrink-0 mt-0.5"
                        style={{ background: `${insight.color}15` }}>
                        <insight.icon className="w-3.5 h-3.5" style={{ color: insight.color }} />
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="text-xs font-medium text-white/70">{lang === 'he' ? insight.titleHe : insight.title}</div>
                        <div className="text-[10px] text-white/25 mt-0.5">{insight.descriptionHe}</div>
                      </div>
                      {insight.priority >= 8 && (
                        <motion.div
                          className="w-2 h-2 rounded-full flex-shrink-0 mt-1.5"
                          style={{ background: insight.color, boxShadow: `0 0 6px ${insight.color}50` }}
                          animate={{ scale: [1, 1.3, 1] }}
                          transition={{ duration: 1.5, repeat: Infinity }}
                        />
                      )}
                    </div>
                  </motion.div>
                ))
              )}
            </motion.div>
          ) : (
            <motion.div
              key="chat"
              initial={{ opacity: 0, x: 10 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -10 }}
              className="space-y-2 min-h-[200px]"
            >
              {messages.length === 0 && (
                <div className="text-center py-8" style={{ direction: dir }}>
                  <Brain className="w-8 h-8 mx-auto mb-2" style={{ color: 'rgba(0,212,255,0.2)' }} />
                  <div className="text-xs text-white/25 mb-1">{t('ai.greeting')}</div>
                  <div className="text-[10px] text-white/12">{t('ai.subtitle')}</div>
                </div>
              )}
              {messages.map((msg, i) => (
                <motion.div
                  key={msg.id}
                  initial={{ opacity: 0, y: 8 }}
                  animate={{ opacity: 1, y: 0 }}
                  className={`flex ${msg.role === 'user' ? 'justify-start' : 'justify-end'}`}
                  style={{ direction: dir }}
                >
                  <div
                    className="max-w-[85%] px-3 py-2 rounded-xl text-xs leading-relaxed"
                    style={{
                      background: msg.role === 'user' ? 'rgba(229,231,235,0.4)' : 'rgba(0,212,255,0.06)',
                      border: msg.role === 'user' ? '1px solid rgba(229,231,235,0.6)' : '1px solid rgba(0,212,255,0.12)',
                      color: msg.role === 'user' ? 'rgba(75,85,99,0.9)' : 'rgba(55,65,81,0.9)' }}
                  >
                    {msg.text}
                  </div>
                </motion.div>
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* ── Chat Input ── */}
      {activeTab === 'chat' && (
        <div className="px-3 pb-3 pt-1" style={{ borderTop: '1px solid rgba(243,244,246,0.5)' }}>
          <div className="flex items-center gap-2">
            <motion.button
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.85 }}
              onClick={() => setIsListening(!isListening)}
              className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer flex-shrink-0"
              style={{
                background: isListening ? 'rgba(255,68,102,0.1)' : 'rgba(229,231,235,0.4)',
                border: isListening ? '1px solid rgba(255,68,102,0.2)' : '1px solid rgba(229,231,235,0.6)' }}
            >
              {isListening ? <MicOff className="w-3.5 h-3.5" style={{ color: '#ff4466' }} /> : <Mic className="w-3.5 h-3.5 text-white/25" />}
            </motion.button>
            <input
              type="text"
              value={input}
              onChange={e => setInput(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && sendMessage()}
              placeholder={t('ai.placeholder')}
              className="flex-1 py-2 px-3 rounded-lg text-xs text-white/70 placeholder:text-white/15 outline-none"
              style={{
                background: 'rgba(243,244,246,0.5)',
                border: '1px solid rgba(229,231,235,0.5)',
                direction: dir }}
            />
            <motion.button
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.85 }}
              onClick={sendMessage}
              className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer flex-shrink-0"
              style={{ background: input.trim() ? 'rgba(0,212,255,0.15)' : 'rgba(229,231,235,0.4)', border: '1px solid rgba(0,212,255,0.2)' }}
            >
              <Send className="w-3.5 h-3.5" style={{ color: input.trim() ? '#2563EB' : 'rgba(209,213,219,0.8)' }} />
            </motion.button>
          </div>
        </div>
      )}
    </motion.div>
  );
}

// ═══════════════════════════════════════════════════
// Ambient Toast — non-intrusive contextual notifications
// ═══════════════════════════════════════════════════
export function AmbientToast({ suggestion, onDismiss }: { suggestion: AISuggestion | null; onDismiss: () => void }) {
  const { t, dir, lang } = useLanguage();
  useEffect(() => {
    if (suggestion) {
      const timer = setTimeout(onDismiss, 6000);
      return () => clearTimeout(timer);
    }
  }, [suggestion, onDismiss]);

  return (
    <AnimatePresence>
      {suggestion && (
        <motion.div
          initial={{ opacity: 0, y: -20, x: '-50%' }}
          animate={{ opacity: 1, y: 0, x: '-50%' }}
          exit={{ opacity: 0, y: -20, x: '-50%' }}
          className="fixed top-4 left-1/2 z-50 flex items-center gap-3 px-4 py-3 rounded-xl cursor-pointer"
          style={{
            background: 'rgba(6,8,14,0.92)',
            border: `1px solid ${suggestion.color}20`,
            boxShadow: `0 4px 24px rgba(0,0,0,0.4), 0 0 16px ${suggestion.color}10`,
            direction: dir }}
          onClick={onDismiss}
        >
          <div className="w-8 h-8 rounded-lg flex items-center justify-center"
            style={{ background: `${suggestion.color}12` }}>
            <suggestion.icon className="w-4 h-4" style={{ color: suggestion.color }} />
          </div>
          <div>
            <div className="text-xs font-medium text-white/70">{lang === 'he' ? suggestion.titleHe : suggestion.title}</div>
            <div className="text-[10px] text-white/25">{suggestion.descriptionHe}</div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

export default AIChatPanel;
