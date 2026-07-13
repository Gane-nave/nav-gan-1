/**
 * G.A.N.E — Full Specification Viewer
 * 36 sections with Hebrew/English, voice TTS, layer grouping
 * Design: Dark Intelligence Dashboard — Aerospace HUD
 */
import { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  ChevronRight, ChevronDown, Search, X, Menu, Home, ArrowLeft,
  ArrowRight, Volume2, Layers, Hash, Globe
} from "lucide-react";
import { Link, useParams } from "wouter";
import { ganeSections, ganeLayers, type GaneSection } from "@/lib/ganeData";
import { useLanguage } from "@/contexts/LanguageContext";
import { useVoice } from "@/contexts/VoiceContext";
import LanguageSelector from "@/components/LanguageSelector";

const colorAccent: Record<string, string> = {
  indigo: "text-indigo-400", teal: "text-teal-400", amber: "text-amber-400",
  green: "text-green-400", rose: "text-rose-400", cyan: "text-cyan-400", violet: "text-violet-400" };
const colorBg: Record<string, string> = {
  indigo: "bg-indigo-500/10 border-indigo-500/20", teal: "bg-teal-500/10 border-teal-500/20",
  amber: "bg-amber-500/10 border-amber-500/20", green: "bg-green-500/10 border-green-500/20",
  rose: "bg-rose-500/10 border-rose-500/20", cyan: "bg-cyan-500/10 border-cyan-500/20",
  violet: "bg-violet-500/10 border-violet-500/20" };
const colorDot: Record<string, string> = {
  indigo: "bg-indigo-400", teal: "bg-teal-400", amber: "bg-amber-400",
  green: "bg-green-400", rose: "bg-rose-400", cyan: "bg-cyan-400", violet: "bg-violet-400" };

function SectionContent({ section, isHe }: { section: GaneSection; isHe: boolean }) {
  const voice = useVoice();
  const { lang } = useLanguage();

  const readSection = () => {
    const text = isHe
      ? `${section.titleHe}. ${section.descriptionHe}. ${section.features.map(f => f.nameHe).join(', ')}`
      : `${section.titleEn}. ${section.features.map(f => f.nameEn).join(', ')}`;
    voice.speak(text, isHe ? 'he' : lang);
  };

  return (
    <motion.div
      key={section.id}
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
    >
      {/* Header */}
      <div className="mb-10 pb-8 border-b border-white/8">
        <div className="flex items-center gap-3 mb-4 flex-wrap">
          <span className="section-badge">§{section.number}</span>
          <span className={`font-['JetBrains_Mono'] text-xs px-2 py-0.5 rounded border ${colorBg[section.color]} ${colorAccent[section.color]}`}>
            {section.features.length} features
          </span>
          {section.layer && (
            <span className="font-['JetBrains_Mono'] text-[10px] px-2 py-0.5 rounded border border-white/8 bg-white/3 text-white/40">
              <Layers className="w-3 h-3 inline mr-1" />
              {isHe ? section.layer : ganeLayers.find(l => l.sections.includes(section.number))?.nameEn || section.layer}
            </span>
          )}
          {voice.isSupported && (
            <button onClick={readSection} className="voice-btn ml-auto">
              <Volume2 className="w-3.5 h-3.5" />
              {isHe ? 'קריאה בקול' : 'Read Aloud'}
            </button>
          )}
        </div>

        <h1 className="font-['Syne'] text-3xl md:text-4xl font-extrabold text-white mb-2 leading-tight">
          {isHe ? section.titleHe : section.titleEn}
        </h1>
        {isHe && (
          <div className="text-sm text-white/35 font-['JetBrains_Mono'] mb-3">{section.titleEn}</div>
        )}
        <p className={`text-white/50 text-sm leading-relaxed max-w-2xl ${isHe ? 'text-right' : ''}`} dir={isHe ? 'rtl' : 'ltr'}>
          {section.descriptionHe}
        </p>
      </div>

      {/* Features Grid */}
      <div className="mb-10">
        <h2 className="font-['Syne'] text-lg font-bold text-white/80 mb-4 flex items-center gap-2">
          <Hash className="w-4 h-4 text-indigo-400" />
          {isHe ? 'רכיבים ויכולות' : 'Components & Features'}
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {section.features.map((feature, i) => (
            <motion.div
              key={i}
              initial={{ opacity: 0, y: 5 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.02 }}
              className="gane-feature-card"
            >
              <div className="flex items-start gap-3">
                <span className={`mt-1 w-1.5 h-1.5 rounded-full flex-shrink-0 ${colorDot[section.color]}`} />
                <div className="flex-1 min-w-0">
                  <div className="font-['Syne'] text-sm font-semibold text-white/85">
                    {isHe ? feature.nameHe : feature.nameEn}
                  </div>
                  {isHe && (
                    <div className="text-xs text-white/30 font-['JetBrains_Mono'] mt-0.5">{feature.nameEn}</div>
                  )}
                  {feature.description && (
                    <div className="text-xs text-white/40 mt-1">{feature.description}</div>
                  )}
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </motion.div>
  );
}

function GaneSidebar({
  activeSection,
  onSelectSection,
  searchQuery,
  onSearchChange,
  sidebarOpen,
  onCloseSidebar,
  viewMode,
  onViewModeChange,
  isHe }: {
  activeSection: string;
  onSelectSection: (id: string) => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  sidebarOpen: boolean;
  onCloseSidebar: () => void;
  viewMode: 'list' | 'layers';
  onViewModeChange: (mode: 'list' | 'layers') => void;
  isHe: boolean;
}) {
  const [expandedLayers, setExpandedLayers] = useState<Set<number>>(new Set([0, 1, 2, 3, 4]));

  const toggleLayer = (idx: number) => {
    setExpandedLayers(prev => {
      const next = new Set(prev);
      if (next.has(idx)) next.delete(idx); else next.add(idx);
      return next;
    });
  };

  const filtered = searchQuery
    ? ganeSections.filter(s =>
        s.titleHe.includes(searchQuery) ||
        s.titleEn.toLowerCase().includes(searchQuery.toLowerCase()) ||
        s.features.some(f => f.nameEn.toLowerCase().includes(searchQuery.toLowerCase()) || f.nameHe.includes(searchQuery))
      )
    : ganeSections;

  return (
    <>
      {sidebarOpen && (
        <div className="fixed inset-0 bg-black/60 z-30 lg:hidden" onClick={onCloseSidebar} />
      )}
      <aside className={`
        fixed top-0 left-0 bottom-0 z-40 w-72 flex flex-col
        bg-[oklch(0.10_0.016_264)] border-r border-white/5
        transition-transform duration-300
        ${sidebarOpen ? "translate-x-0" : "-translate-x-full lg:translate-x-0"}
      `}>
        {/* Header */}
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

        {/* Search */}
        <div className="px-3 py-3 border-b border-white/5 flex-shrink-0">
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/25" />
            <input
              type="text"
              placeholder={isHe ? 'חיפוש...' : 'Search sections...'}
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

        {/* View mode toggle */}
        <div className="px-3 py-2 border-b border-white/5 flex gap-1 flex-shrink-0">
          <button
            onClick={() => onViewModeChange('list')}
            className={`flex-1 py-1.5 text-[10px] font-['JetBrains_Mono'] rounded transition-all ${viewMode === 'list' ? 'bg-indigo-500/15 text-indigo-300 border border-indigo-500/25' : 'text-white/30 hover:text-white/50 border border-transparent'}`}
          >
            {isHe ? 'רשימה' : 'LIST'}
          </button>
          <button
            onClick={() => onViewModeChange('layers')}
            className={`flex-1 py-1.5 text-[10px] font-['JetBrains_Mono'] rounded transition-all ${viewMode === 'layers' ? 'bg-indigo-500/15 text-indigo-300 border border-indigo-500/25' : 'text-white/30 hover:text-white/50 border border-transparent'}`}
          >
            {isHe ? 'שכבות' : 'LAYERS'}
          </button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 overflow-y-auto py-2">
          {viewMode === 'list' ? (
            filtered.map(section => {
              const isActive = activeSection === section.id;
              return (
                <button
                  key={section.id}
                  onClick={() => { onSelectSection(section.id); onCloseSidebar(); }}
                  className={`w-full flex items-center gap-2 px-3 py-2 text-left transition-all ${
                    isActive
                      ? 'bg-indigo-500/10 border-l-2 border-indigo-500 text-white/90'
                      : 'border-l-2 border-transparent text-white/45 hover:text-white/70 hover:bg-white/3'
                  }`}
                >
                  <span className={`font-['JetBrains_Mono'] text-[10px] w-6 flex-shrink-0 ${colorAccent[section.color]}`}>
                    {section.number}
                  </span>
                  <span className="font-['Syne'] text-xs font-semibold flex-1 leading-tight truncate">
                    {isHe ? section.titleHe : section.titleEn}
                  </span>
                  <span className="text-[9px] text-white/20 flex-shrink-0">{section.features.length}</span>
                </button>
              );
            })
          ) : (
            ganeLayers.map((layer, li) => (
              <div key={li}>
                <button
                  onClick={() => toggleLayer(li)}
                  className="w-full flex items-center gap-2 px-3 py-2.5 text-left text-white/60 hover:text-white/80 transition-all"
                >
                  <Layers className="w-3.5 h-3.5 text-indigo-400 flex-shrink-0" />
                  <span className="font-['Syne'] text-xs font-semibold flex-1 leading-tight">
                    {isHe ? layer.nameHe : layer.nameEn}
                  </span>
                  {expandedLayers.has(li) ? <ChevronDown className="w-3 h-3 opacity-40" /> : <ChevronRight className="w-3 h-3 opacity-40" />}
                </button>
                <AnimatePresence>
                  {expandedLayers.has(li) && (
                    <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: "auto", opacity: 1 }} exit={{ height: 0, opacity: 0 }} className="overflow-hidden">
                      {layer.sections.map(sNum => {
                        const section = ganeSections.find(s => s.number === sNum);
                        if (!section) return null;
                        const isActive = activeSection === section.id;
                        return (
                          <button
                            key={section.id}
                            onClick={() => { onSelectSection(section.id); onCloseSidebar(); }}
                            className={`w-full flex items-center gap-2 pl-8 pr-3 py-1.5 text-left transition-all ${
                              isActive ? 'text-indigo-300 bg-indigo-500/8' : 'text-white/35 hover:text-white/60'
                            }`}
                          >
                            <span className={`font-['JetBrains_Mono'] text-[9px] w-5 ${colorAccent[section.color]}`}>{section.number}</span>
                            <span className="text-xs truncate">{isHe ? section.titleHe : section.titleEn}</span>
                          </button>
                        );
                      })}
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            ))
          )}
        </nav>

        {/* Footer */}
        <div className="px-4 py-3 border-t border-white/5 flex-shrink-0 flex items-center justify-between">
          <div className="font-['JetBrains_Mono'] text-[10px] text-white/20">
            G.A.N.E v2.0
          </div>
          <Link href="/spec">
            <span className="text-[10px] text-indigo-400 hover:text-indigo-300 cursor-pointer transition-colors">G.A.N.E Spec →</span>
          </Link>
        </div>
      </aside>
    </>
  );
}

export default function GanePage() {
  const params = useParams<{ sectionId?: string }>();
  const { lang, t } = useLanguage();
  const isHe = lang === 'he' || lang === 'ar' || lang === 'fa' || lang === 'ur';
  const [activeSection, setActiveSection] = useState(params.sectionId || "gane-1");
  const [searchQuery, setSearchQuery] = useState("");
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [viewMode, setViewMode] = useState<'list' | 'layers'>('list');
  const contentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (params.sectionId) setActiveSection(params.sectionId);
  }, [params.sectionId]);

  const currentSection = ganeSections.find(s => s.id === activeSection) || ganeSections[0];
  const currentIndex = ganeSections.findIndex(s => s.id === activeSection);
  const prevSection = currentIndex > 0 ? ganeSections[currentIndex - 1] : null;
  const nextSection = currentIndex < ganeSections.length - 1 ? ganeSections[currentIndex + 1] : null;

  const handleSelectSection = (id: string) => {
    setActiveSection(id);
    if (contentRef.current) contentRef.current.scrollTop = 0;
  };

  return (
    <div className="min-h-screen bg-background text-foreground flex">
      <GaneSidebar
        activeSection={activeSection}
        onSelectSection={handleSelectSection}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        sidebarOpen={sidebarOpen}
        onCloseSidebar={() => setSidebarOpen(false)}
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        isHe={isHe}
      />

      <div className="flex-1 lg:ml-72 flex flex-col min-h-screen">
        {/* Top bar */}
        <header className="sticky top-0 z-20 h-14 border-b border-white/5 bg-background/90 backdrop-blur-xl flex items-center px-4 md:px-8 gap-4">
          <button onClick={() => setSidebarOpen(true)} className="lg:hidden text-white/40 hover:text-white/70 transition-colors">
            <Menu className="w-5 h-5" />
          </button>

          <div className="flex items-center gap-2 text-xs text-white/30 font-['JetBrains_Mono']">
            <Link href="/"><span className="hover:text-white/60 cursor-pointer transition-colors">HOME</span></Link>
            <ChevronRight className="w-3 h-3" />
            <span className="text-white/50">G.A.N.E</span>
            <ChevronRight className="w-3 h-3" />
            <span className={colorAccent[currentSection.color]}>§{currentSection.number}</span>
          </div>

          <div className="ml-auto flex items-center gap-3">
            <LanguageSelector compact />
            <span className="hidden md:block text-xs text-white/25 font-['JetBrains_Mono']">
              {currentIndex + 1} / {ganeSections.length}
            </span>
            <Link href="/">
              <button className="flex items-center gap-1.5 px-3 py-1.5 text-xs text-white/40 hover:text-white/70 border border-white/8 hover:border-white/20 rounded transition-all">
                <Home className="w-3 h-3" /> {t('nav.home')}
              </button>
            </Link>
          </div>
        </header>

        {/* Content */}
        <main ref={contentRef} className="flex-1 overflow-y-auto">
          <div className="max-w-4xl mx-auto px-4 md:px-8 py-10">
            <SectionContent section={currentSection} isHe={isHe} />

            {/* Navigation */}
            <div className="flex items-center justify-between pt-8 border-t border-white/8 mt-4">
              {prevSection ? (
                <button
                  onClick={() => handleSelectSection(prevSection.id)}
                  className="flex items-center gap-2 px-4 py-2.5 border border-white/10 hover:border-white/25 rounded text-sm text-white/50 hover:text-white/80 transition-all"
                >
                  <ArrowLeft className="w-4 h-4" />
                  <div className="text-left">
                    <div className="text-xs text-white/30 font-['JetBrains_Mono']">{t('spec.previous')}</div>
                    <div className="font-['Syne'] text-sm">{isHe ? prevSection.titleHe : prevSection.titleEn}</div>
                  </div>
                </button>
              ) : <div />}
              {nextSection ? (
                <button
                  onClick={() => handleSelectSection(nextSection.id)}
                  className="flex items-center gap-2 px-4 py-2.5 border border-white/10 hover:border-indigo-500/30 rounded text-sm text-white/50 hover:text-white/80 transition-all ml-auto"
                >
                  <div className="text-right">
                    <div className="text-xs text-white/30 font-['JetBrains_Mono']">{t('spec.next')}</div>
                    <div className="font-['Syne'] text-sm">{isHe ? nextSection.titleHe : nextSection.titleEn}</div>
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
