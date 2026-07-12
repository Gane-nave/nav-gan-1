/**
 * LanguageSelector — Dropdown for selecting UI language
 * Shows flag + native name, supports search
 */
import { useState, useRef, useEffect } from 'react';
import { useLanguage } from '@/contexts/LanguageContext';
import { languages } from '@/lib/i18n';
import { Globe, ChevronDown, Search, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export default function LanguageSelector({ compact = false }: { compact?: boolean }) {
  const { lang, setLang, language } = useLanguage();
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState('');
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, []);

  const filtered = search
    ? languages.filter(l =>
        l.name.toLowerCase().includes(search.toLowerCase()) ||
        l.nativeName.toLowerCase().includes(search.toLowerCase()) ||
        l.code.includes(search.toLowerCase())
      )
    : languages;

  return (
    <div ref={ref} className="relative">
      <button
        onClick={() => setOpen(!open)}
        className="flex items-center gap-1.5 px-2.5 py-1.5 bg-white/5 border border-white/10 rounded-lg text-xs text-white/60 hover:text-white/80 hover:border-white/20 transition-all"
      >
        <Globe className="w-3.5 h-3.5" />
        {!compact && <span>{language.nativeName}</span>}
        <ChevronDown className={`w-3 h-3 transition-transform ${open ? 'rotate-180' : ''}`} />
      </button>

      <AnimatePresence>
        {open && (
          <motion.div
            initial={{ opacity: 0, y: -8, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -8, scale: 0.95 }}
            transition={{ duration: 0.15 }}
            className="absolute top-full mt-2 right-0 w-64 max-h-80 bg-[oklch(0.12_0.02_264)] border border-white/10 rounded-xl shadow-2xl shadow-black/50 overflow-hidden z-50"
          >
            {/* Search */}
            <div className="p-2 border-b border-white/5">
              <div className="relative">
                <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3 h-3 text-white/25" />
                <input
                  type="text"
                  value={search}
                  onChange={e => setSearch(e.target.value)}
                  placeholder="Search language..."
                  className="w-full pl-7 pr-7 py-1.5 bg-white/5 border border-white/8 rounded-lg text-xs text-white/70 placeholder-white/25 focus:outline-none focus:border-indigo-500/40"
                  autoFocus
                />
                {search && (
                  <button onClick={() => setSearch('')} className="absolute right-2.5 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/60">
                    <X className="w-3 h-3" />
                  </button>
                )}
              </div>
            </div>

            {/* Language list */}
            <div className="overflow-y-auto max-h-60 py-1">
              {filtered.map(l => (
                <button
                  key={l.code}
                  onClick={() => { setLang(l.code); setOpen(false); setSearch(''); }}
                  className={`w-full flex items-center gap-3 px-3 py-2 text-left transition-all ${
                    l.code === lang
                      ? 'bg-indigo-500/15 text-indigo-300'
                      : 'text-white/50 hover:text-white/80 hover:bg-white/5'
                  }`}
                >
                  <span className="text-base w-6 text-center">{l.flag}</span>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-medium truncate">{l.nativeName}</div>
                    <div className="text-[10px] text-white/30">{l.name}</div>
                  </div>
                  {l.dir === 'rtl' && (
                    <span className="text-[9px] px-1.5 py-0.5 bg-amber-500/10 border border-amber-500/20 rounded text-amber-400">
                      RTL
                    </span>
                  )}
                  {l.code === lang && (
                    <div className="w-1.5 h-1.5 rounded-full bg-indigo-400" />
                  )}
                </button>
              ))}
              {filtered.length === 0 && (
                <div className="px-3 py-4 text-xs text-white/30 text-center">No languages found</div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
