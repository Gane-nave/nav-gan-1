/**
 * G.A.N.E Specification Portal — Spec Viewer Page
 * Design: Dark Intelligence Dashboard — Aerospace HUD aesthetic
 * Full sidebar navigation + main content area with section rendering
 * Integrated with voice TTS and i18n
 */
import { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { ChevronRight, ChevronDown, Search, X, Menu, Home, ArrowLeft, Volume2 } from "lucide-react";
import { Link, useParams } from "wouter";
import { specParts, type SpecPart, type SpecSection } from "@/lib/specData";
import { useLanguage } from "@/contexts/LanguageContext";
import { useVoice } from "@/contexts/VoiceContext";
import LanguageSelector from "@/components/LanguageSelector";

const colorAccent: Record<string, string> = {
  indigo: "text-indigo-400 border-indigo-500", teal: "text-teal-400 border-teal-500",
  amber: "text-amber-400 border-amber-500", green: "text-green-400 border-green-500",
  rose: "text-rose-400 border-rose-500" };
const colorBg: Record<string, string> = {
  indigo: "bg-indigo-500/10 border-indigo-500/20", teal: "bg-teal-500/10 border-teal-500/20",
  amber: "bg-amber-500/10 border-amber-500/20", green: "bg-green-500/10 border-green-500/20",
  rose: "bg-rose-500/10 border-rose-500/20" };

function SectionContent({ section, partColor }: { section: SpecSection; partColor: string }) {
  return (
    <div className="mb-12">
      <div className="flex items-baseline gap-3 mb-4">
        <span className={`font-['JetBrains_Mono'] text-sm font-medium ${colorAccent[partColor]}`}>
          §{section.number}
        </span>
        <h2 className="font-['Syne'] text-xl font-bold text-white">{section.title}</h2>
      </div>
      {section.content && (
        <p className="text-white/60 leading-relaxed mb-6 text-sm">{section.content}</p>
      )}
      {section.subsections?.map((sub) => (
        <div key={sub.id} className="ml-0 mb-8">
          <div className="flex items-baseline gap-3 mb-3">
            <span className={`font-['JetBrains_Mono'] text-xs ${colorAccent[partColor]} opacity-70`}>
              {sub.number}
            </span>
            <h3 className="font-['Syne'] text-base font-semibold text-white/85">{sub.title}</h3>
          </div>
          {sub.content && (
            <p className="text-white/55 text-sm leading-relaxed mb-4">{sub.content}</p>
          )}
          {sub.code && (
            <div className="code-block mb-4 overflow-x-auto">
              <pre className="text-xs leading-relaxed whitespace-pre">{sub.code}</pre>
            </div>
          )}
          {sub.table && (
            <div className="overflow-x-auto mb-4 rounded border border-white/8">
              <table className="spec-table">
                <thead>
                  <tr>{sub.table.headers.map((h) => <th key={h}>{h}</th>)}</tr>
                </thead>
                <tbody>
                  {sub.table.rows.map((row, ri) => (
                    <tr key={ri}>
                      {row.map((cell, ci) => (
                        <td key={ci} className={ci === 0 ? "font-['JetBrains_Mono'] text-xs text-white/70" : ""}>{cell}</td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
          {sub.items && (
            <ul className="space-y-2">
              {sub.items.map((item, ii) => (
                <li key={ii} className="flex items-start gap-2 text-sm text-white/55">
                  <span className={`mt-1.5 w-1 h-1 rounded-full flex-shrink-0 ${colorAccent[partColor].replace('text-', 'bg-')}`} />
                  <span>{item}</span>
                </li>
              ))}
            </ul>
          )}
        </div>
      ))}
    </div>
  );
}

function Sidebar({
  activePart, onSelectPart, searchQuery, onSearchChange, sidebarOpen, onCloseSidebar }: {
  activePart: string; onSelectPart: (id: string) => void;
  searchQuery: string; onSearchChange: (q: string) => void;
  sidebarOpen: boolean; onCloseSidebar: () => void;
}) {
  const { t } = useLanguage();
  const [expandedParts, setExpandedParts] = useState<Set<string>>(new Set([activePart]));

  useEffect(() => {
    setExpandedParts(prev => new Set(Array.from(prev).concat([activePart])));
  }, [activePart]);

  const togglePart = (id: string) => {
    setExpandedParts(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id); else next.add(id);
      return next;
    });
  };

  const filteredParts = searchQuery
    ? specParts.filter(p =>
        p.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        p.sections.some(s => s.title.toLowerCase().includes(searchQuery.toLowerCase()))
      )
    : specParts;

  return (
    <>
      {sidebarOpen && <div className="fixed inset-0 bg-black/60 z-30 lg:hidden" onClick={onCloseSidebar} />}
      <aside className={`
        fixed top-0 left-0 bottom-0 z-40 w-72 flex flex-col
        bg-[oklch(0.10_0.016_264)] border-r border-white/5
        transition-transform duration-300
        ${sidebarOpen ? "translate-x-0" : "-translate-x-full lg:translate-x-0"}
      `}>
        <div className="flex items-center justify-between px-4 h-14 border-b border-white/5 flex-shrink-0">
          <Link href="/">
            <div className="flex items-center gap-2 cursor-pointer group">
              <div className="w-6 h-6 rounded bg-indigo-500/20 border border-indigo-500/40 flex items-center justify-center">
                <div className="w-2.5 h-2.5 rounded-sm bg-indigo-400" />
              </div>
              <span className="font-['Syne'] font-bold text-sm text-white/80 group-hover:text-white transition-colors">G.A.N.E</span>
            </div>
          </Link>
          <button onClick={onCloseSidebar} className="lg:hidden text-white/40 hover:text-white/70">
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="px-3 py-3 border-b border-white/5 flex-shrink-0">
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/25" />
            <input
              type="text"
              placeholder={t('spec.search')}
              value={searchQuery}
              onChange={e => onSearchChange(e.target.value)}
              className="w-full pl-8 pr-3 py-2 bg-white/5 border border-white/8 rounded text-xs text-white/70 placeholder-white/25 focus:outline-none focus:border-indigo-500/40 transition-all"
            />
            {searchQuery && (
              <button onClick={() => onSearchChange("")} className="absolute right-2.5 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/60">
                <X className="w-3 h-3" />
              </button>
            )}
          </div>
        </div>

        <nav className="flex-1 overflow-y-auto py-2">
          {filteredParts.map((part) => {
            const isActive = activePart === part.id;
            const isExpanded = expandedParts.has(part.id);
            const accent = colorAccent[part.color];
            return (
              <div key={part.id}>
                <button
                  onClick={() => { onSelectPart(part.id); togglePart(part.id); }}
                  className={`w-full flex items-center gap-2 px-3 py-2.5 text-left transition-all ${
                    isActive ? 'bg-indigo-500/10 border-l-2 border-indigo-500 text-white/90' : 'border-l-2 border-transparent text-white/50 hover:text-white/75 hover:bg-white/3'
                  }`}
                >
                  <span className={`font-['JetBrains_Mono'] text-[10px] font-medium w-8 flex-shrink-0 ${accent}`}>{part.romanNumeral}</span>
                  <span className="font-['Syne'] text-xs font-semibold flex-1 leading-tight">{part.title}</span>
                  {isExpanded ? <ChevronDown className="w-3 h-3 flex-shrink-0 opacity-40" /> : <ChevronRight className="w-3 h-3 flex-shrink-0 opacity-40" />}
                </button>
                <AnimatePresence>
                  {isExpanded && (
                    <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: "auto", opacity: 1 }} exit={{ height: 0, opacity: 0 }} transition={{ duration: 0.2 }} className="overflow-hidden">
                      {part.sections.map((section) => (
                        <a key={section.id} href={`#${section.id}`} className="flex items-center gap-2 pl-10 pr-3 py-1.5 text-white/35 hover:text-white/60 transition-colors">
                          <span className="font-['JetBrains_Mono'] text-[9px] text-white/20 w-5">§{section.number}</span>
                          <span className="text-xs leading-tight">{section.title}</span>
                        </a>
                      ))}
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            );
          })}
        </nav>

        <div className="px-4 py-3 border-t border-white/5 flex-shrink-0 flex items-center justify-between">
          <div className="font-['JetBrains_Mono'] text-[10px] text-white/20">G.A.N.E SPEC v2.0</div>
          <Link href="/gane">
            <span className="text-[10px] text-indigo-400 hover:text-indigo-300 cursor-pointer transition-colors">G.A.N.E →</span>
          </Link>
        </div>
      </aside>
    </>
  );
}

function PartContent({ part }: { part: SpecPart }) {
  const voice = useVoice();
  const { lang, t } = useLanguage();

  const readPart = () => {
    const text = `${part.title}. ${part.description}. ${part.sections.map(s => s.title).join('. ')}`;
    voice.speak(text, lang);
  };

  return (
    <motion.div key={part.id} initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.3 }}>
      <div className="mb-10 pb-8 border-b border-white/8">
        <div className="flex items-center gap-3 mb-4 flex-wrap">
          <span className="section-badge">Part {part.romanNumeral}</span>
          <span className={`font-['JetBrains_Mono'] text-xs px-2 py-0.5 rounded border ${colorBg[part.color]} ${colorAccent[part.color]}`}>
            {part.sections.length} sections
          </span>
          {voice.isSupported && (
            <button onClick={readPart} className="voice-btn ml-auto">
              <Volume2 className="w-3.5 h-3.5" />
              {t('voice.play')}
            </button>
          )}
        </div>
        <h1 className="font-['Syne'] text-3xl md:text-4xl font-extrabold text-white mb-3 leading-tight">{part.title}</h1>
        <p className="text-white/45 text-sm leading-relaxed max-w-2xl">{part.description}</p>
      </div>
      {part.sections.map((section) => (
        <div key={section.id} id={section.id}>
          <SectionContent section={section} partColor={part.color} />
          <div className="border-t border-white/5 mb-10" />
        </div>
      ))}
    </motion.div>
  );
}

export default function SpecPage() {
  const params = useParams<{ partId?: string }>();
  const { t } = useLanguage();
  const [activePart, setActivePart] = useState(params.partId || "part-i");
  const [searchQuery, setSearchQuery] = useState("");
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const contentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (params.partId) setActivePart(params.partId);
  }, [params.partId]);

  const currentPart = specParts.find(p => p.id === activePart) || specParts[0];
  const handleSelectPart = (id: string) => {
    setActivePart(id);
    setSidebarOpen(false);
    if (contentRef.current) contentRef.current.scrollTop = 0;
  };

  const currentIndex = specParts.findIndex(p => p.id === activePart);
  const prevPart = currentIndex > 0 ? specParts[currentIndex - 1] : null;
  const nextPart = currentIndex < specParts.length - 1 ? specParts[currentIndex + 1] : null;

  return (
    <div className="min-h-screen bg-background text-foreground flex">
      <Sidebar
        activePart={activePart} onSelectPart={handleSelectPart}
        searchQuery={searchQuery} onSearchChange={setSearchQuery}
        sidebarOpen={sidebarOpen} onCloseSidebar={() => setSidebarOpen(false)}
      />
      <div className="flex-1 lg:ml-72 flex flex-col min-h-screen">
        <header className="sticky top-0 z-20 h-14 border-b border-white/5 bg-background/90 backdrop-blur-xl flex items-center px-4 md:px-8 gap-4">
          <button onClick={() => setSidebarOpen(true)} className="lg:hidden text-white/40 hover:text-white/70 transition-colors">
            <Menu className="w-5 h-5" />
          </button>
          <div className="flex items-center gap-2 text-xs text-white/30 font-['JetBrains_Mono']">
            <Link href="/"><span className="hover:text-white/60 cursor-pointer transition-colors">HOME</span></Link>
            <ChevronRight className="w-3 h-3" />
            <span className="text-white/50">SPEC</span>
            <ChevronRight className="w-3 h-3" />
            <span className={`${colorAccent[currentPart.color]}`}>PART {currentPart.romanNumeral}</span>
          </div>
          <div className="ml-auto flex items-center gap-3">
            <LanguageSelector compact />
            <span className="hidden md:block text-xs text-white/25 font-['JetBrains_Mono']">{currentIndex + 1} / {specParts.length}</span>
            <Link href="/">
              <button className="flex items-center gap-1.5 px-3 py-1.5 text-xs text-white/40 hover:text-white/70 border border-white/8 hover:border-white/20 rounded transition-all">
                <Home className="w-3 h-3" /> {t('nav.home')}
              </button>
            </Link>
          </div>
        </header>

        <main ref={contentRef} className="flex-1 overflow-y-auto">
          <div className="max-w-4xl mx-auto px-4 md:px-8 py-10">
            <PartContent part={currentPart} />
            <div className="flex items-center justify-between pt-8 border-t border-white/8 mt-4">
              {prevPart ? (
                <button onClick={() => handleSelectPart(prevPart.id)} className="flex items-center gap-2 px-4 py-2.5 border border-white/10 hover:border-white/25 rounded text-sm text-white/50 hover:text-white/80 transition-all">
                  <ArrowLeft className="w-4 h-4" />
                  <div className="text-left">
                    <div className="text-xs text-white/30 font-['JetBrains_Mono']">{t('spec.previous')}</div>
                    <div className="font-['Syne'] text-sm">{prevPart.title}</div>
                  </div>
                </button>
              ) : <div />}
              {nextPart ? (
                <button onClick={() => handleSelectPart(nextPart.id)} className="flex items-center gap-2 px-4 py-2.5 border border-white/10 hover:border-indigo-500/30 rounded text-sm text-white/50 hover:text-white/80 transition-all ml-auto">
                  <div className="text-right">
                    <div className="text-xs text-white/30 font-['JetBrains_Mono']">{t('spec.next')}</div>
                    <div className="font-['Syne'] text-sm">{nextPart.title}</div>
                  </div>
                  <ChevronRight className="w-4 h-4" />
                </button>
              ) : null}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
