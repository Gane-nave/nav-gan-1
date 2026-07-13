/**
 * G.A.N.E — Spec Vault Panel
 * Expandable sections showing full Hebrew + English specification content
 * from all 114 G.A.N.E sections + 15 G.A.N.E parts
 * 10 layer categories, 800+ features — the most comprehensive navigation spec ever created
 */
import { useState, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDown, ChevronRight, FileText, Layers, CircleDot, X, BookOpen, Download, Loader2 } from 'lucide-react';
import { ganeSections } from '@/lib/ganeData';
import { specParts } from '@/lib/specData';
import { toast } from 'sonner';

const colorMap: Record<string, string> = {
  indigo: 'text-gane-indigo', teal: 'text-gane-teal', amber: 'text-gane-amber',
  green: 'text-gane-green', rose: 'text-gane-rose', cyan: 'text-gane-cyan', violet: 'text-gane-violet' };
const bgMap: Record<string, string> = {
  indigo: 'bg-gane-indigo/8', teal: 'bg-gane-teal/8', amber: 'bg-gane-amber/8',
  green: 'bg-gane-green/8', rose: 'bg-gane-rose/8', cyan: 'bg-gane-cyan/8', violet: 'bg-gane-violet/8' };
const borderMap: Record<string, string> = {
  indigo: 'border-gane-indigo/15', teal: 'border-gane-teal/15', amber: 'border-gane-amber/15',
  green: 'border-gane-green/15', rose: 'border-gane-rose/15', cyan: 'border-gane-cyan/15', violet: 'border-gane-violet/15' };

export default function SpecVaultPanel({ onClose }: { onClose: () => void }) {
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'gane' | 'spec'>('gane');
  const [searchQuery, setSearchQuery] = useState('');

  const filteredGane = ganeSections.filter(s =>
    !searchQuery ||
    s.titleEn.toLowerCase().includes(searchQuery.toLowerCase()) ||
    s.titleHe.includes(searchQuery) ||
    s.descriptionHe.includes(searchQuery)
  );

  const filteredSpec = specParts.filter(p =>
    !searchQuery ||
    p.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    p.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
    p.sections.some(sec => sec.title.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  // Group gane sections by layer
  const layerGroups = filteredGane.reduce((acc, section) => {
    const layer = section.layer || 'Other';
    if (!acc[layer]) acc[layer] = [];
    acc[layer].push(section);
    return acc;
  }, {} as Record<string, typeof ganeSections>);

  const totalFeatures = ganeSections.reduce((acc, s) => acc + s.features.length, 0);
  const totalSections = specParts.reduce((acc, p) => acc + p.sections.length, 0);
  const [exporting, setExporting] = useState(false);

  const exportPDF = useCallback(async () => {
    setExporting(true);
    toast.info('Generating PDF specification...');
    try {
      // Build HTML content for PDF
      const layerGroups2 = ganeSections.reduce((acc, section) => {
        const layer = section.layer || 'Other';
        if (!acc[layer]) acc[layer] = [];
        acc[layer].push(section);
        return acc;
      }, {} as Record<string, typeof ganeSections>);

      let html = `
<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;700&family=Syne:wght@600;700;800&display=swap');
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body { font-family: 'Inter', sans-serif; background: #F9FAFB; color: #111827; padding: 40px; }
  h1 { font-family: 'Syne', sans-serif; font-size: 32px; color: #5ce0d8; margin-bottom: 8px; }
  h2 { font-family: 'Syne', sans-serif; font-size: 22px; color: #5ce0d8; margin: 32px 0 12px; border-bottom: 1px solid rgba(229,231,235,0.6); padding-bottom: 8px; }
  h3 { font-family: 'Syne', sans-serif; font-size: 16px; color: #8b8ff5; margin: 20px 0 8px; }
  h4 { font-size: 13px; color: #c0c4d0; margin: 12px 0 6px; }
  p { font-size: 12px; line-height: 1.6; color: #a0a4b0; margin-bottom: 8px; }
  .subtitle { font-size: 14px; color: #6b6f80; margin-bottom: 24px; }
  .section-card { background: rgba(243,244,246,0.5); border: 1px solid rgba(229,231,235,0.5); border-radius: 12px; padding: 16px; margin-bottom: 12px; }
  .feature-list { display: grid; grid-template-columns: 1fr 1fr; gap: 4px 16px; margin-top: 8px; }
  .feature-item { font-size: 11px; color: #8b8ff5; padding: 3px 0; }
  .feature-item::before { content: '\u25B8 '; color: #5ce0d8; }
  .stats { display: flex; gap: 24px; margin: 16px 0 32px; }
  .stat { text-align: center; }
  .stat-value { font-family: 'JetBrains Mono', monospace; font-size: 24px; font-weight: 700; color: #5ce0d8; }
  .stat-label { font-size: 9px; text-transform: uppercase; letter-spacing: 2px; color: #6b6f80; }
  .page-break { page-break-before: always; }
  .layer-badge { display: inline-block; font-size: 10px; padding: 2px 8px; border-radius: 6px; background: rgba(92,224,216,0.1); color: #5ce0d8; margin-bottom: 12px; }
  .gane-section { margin-left: 16px; margin-bottom: 8px; }
  .gane-content { font-size: 11px; color: #8b8ff5; margin-left: 8px; }
  table { width: 100%; border-collapse: collapse; margin: 8px 0; font-size: 11px; }
  th { background: rgba(92,224,216,0.08); color: #5ce0d8; padding: 6px 8px; text-align: left; border: 1px solid rgba(229,231,235,0.6); }
  td { padding: 5px 8px; border: 1px solid rgba(229,231,235,0.4); color: #a0a4b0; }
  .footer { margin-top: 40px; padding-top: 16px; border-top: 1px solid rgba(229,231,235,0.6); text-align: center; font-size: 10px; color: #4b4f60; }
</style></head><body>
<h1>G.A.N.E — Global Autonomous Navigation Ecosystem</h1>
<p class="subtitle">Engineering Master Specification — Generated ${new Date().toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}</p>
<div class="stats">
  <div class="stat"><div class="stat-value">${ganeSections.length}</div><div class="stat-label">G.A.N.E Sections</div></div>
  <div class="stat"><div class="stat-value">${totalSections}</div><div class="stat-label">Sub-Sections</div></div>
  <div class="stat"><div class="stat-value">${totalFeatures}</div><div class="stat-label">Features</div></div>
</div>

<h2>Part A — G.A.N.E Specification</h2>
`;
      // G.A.N.E sections grouped by layer
      Object.entries(layerGroups2).forEach(([layer, sections]) => {
        html += `<div class="layer-badge">${layer}</div>\n`;
        sections.forEach(s => {
          html += `<div class="section-card">
  <h3>${s.number}. ${s.titleEn}</h3>
  <h4 dir="rtl">${s.titleHe}</h4>
  <p dir="rtl">${s.descriptionHe}</p>
  <div class="feature-list">
    ${s.features.map(f => `<div class="feature-item">${f.nameEn} / ${f.nameHe}</div>`).join('\n    ')}
  </div>
</div>\n`;
        });
      });

      html += `<div class="page-break"></div>\n<h2>Part B — G.A.N.E Sub-Specification</h2>\n`;
      specParts.forEach(part => {
        html += `<div class="section-card">
  <h3>Part ${part.romanNumeral}: ${part.title}</h3>
  <p>${part.description}</p>\n`;
        part.sections.forEach(sec => {
          html += `  <div class="gane-section">
    <h4>${sec.number} ${sec.title}</h4>
    <p class="gane-content">${sec.content}</p>\n`;
          if (sec.subsections) {
            sec.subsections.forEach(sub => {
              html += `    <div style="margin-left:16px">
      <h4>${sub.number} ${sub.title}</h4>
      <p class="gane-content">${sub.content}</p>\n`;
              if (sub.items) {
                html += `      <div class="feature-list">${sub.items.map(item => `<div class="feature-item">${item}</div>`).join('')}</div>\n`;
              }
              if (sub.table) {
                html += `      <table><thead><tr>${sub.table.headers.map(h => `<th>${h}</th>`).join('')}</tr></thead><tbody>${sub.table.rows.map(row => `<tr>${row.map(cell => `<td>${cell}</td>`).join('')}</tr>`).join('')}</tbody></table>\n`;
              }
              html += `    </div>\n`;
            });
          }
          html += `  </div>\n`;
        });
        html += `</div>\n`;
      });

      html += `<div class="footer">
  <p>G.A.N.E — Global Autonomous Navigation Ecosystem — Engineering Master Specification</p>
  <p>Confidential — © ${new Date().getFullYear()} G.A.N.E Technologies</p>
</div>\n</body></html>`;

      // Create and download
      const blob = new Blob([html], { type: 'text/html' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `GANE_Specification_${new Date().toISOString().split('T')[0]}.html`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      toast.success('Specification exported successfully!');
    } catch (err) {
      toast.error('Export failed. Please try again.');
    } finally {
      setExporting(false);
    }
  }, [totalSections, totalFeatures]);

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 320 }}
      className="expand-panel scrollbar-none"
    >
      {/* Header */}
      <div className="sticky top-0 z-10 p-4 pb-3" style={{ background: 'oklch(0.09 0.015 264 / 95%)' }}>
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: 'oklch(0.60 0.25 300 / 10%)', border: '1px solid oklch(0.60 0.25 300 / 20%)' }}>
              <BookOpen size={18} className="text-gane-violet" />
            </div>
            <div>
              <h3 className="text-sm font-semibold tracking-wider uppercase text-gane-violet" style={{ fontFamily: 'Syne, sans-serif' }}>Spec Vault</h3>
              <p className="text-[10px] text-muted-foreground">Engineering Master Specification</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={exportPDF}
              disabled={exporting}
              className="w-8 h-8 rounded-lg flex items-center justify-center transition-all hover:bg-gane-violet/10"
              style={{ border: '1px solid oklch(0.60 0.25 300 / 15%)' }}
              aria-label="Export specification"
            >
              {exporting ? <Loader2 size={14} className="text-gane-violet animate-spin" /> : <Download size={14} className="text-gane-violet" />}
            </button>
            <button onClick={onClose} className="fab w-8 h-8" aria-label="Close"><X size={14} /></button>
          </div>
        </div>

        {/* Search */}
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Search specifications..."
          className="search-bar w-full text-xs mb-3"
        />

        {/* Tab Switcher */}
        <div className="flex gap-1 p-1 rounded-xl" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          <button
            onClick={() => setActiveTab('gane')}
            className={`flex-1 py-2 rounded-lg text-xs font-medium transition-all ${
              activeTab === 'gane'
                ? 'text-gane-violet' : 'text-muted-foreground hover:text-foreground'
            }`}
            style={activeTab === 'gane' ? { background: 'oklch(0.60 0.25 300 / 12%)', border: '1px solid oklch(0.60 0.25 300 / 20%)' } : {}}
          >
            G.A.N.E ({ganeSections.length})
          </button>
          <button
            onClick={() => setActiveTab('gane')}
            className={`flex-1 py-2 rounded-lg text-xs font-medium transition-all ${
              activeTab === 'gane'
                ? 'text-gane-cyan' : 'text-muted-foreground hover:text-foreground'
            }`}
            style={activeTab === 'gane' ? { background: 'oklch(0.82 0.15 192 / 12%)', border: '1px solid oklch(0.82 0.15 192 / 20%)' } : {}}
          >
            G.A.N.E ({specParts.length})
          </button>
        </div>
      </div>

      {/* Stats */}
      <div className="px-4 pb-3">
        <div className="grid grid-cols-4 gap-2">
          <div className="feature-card !p-2.5 text-center">
            <div className="text-base font-bold metric-value text-gane-violet">{ganeSections.length}</div>
            <div className="text-[8px] text-muted-foreground uppercase tracking-wider">Sections</div>
          </div>
          <div className="feature-card !p-2.5 text-center">
            <div className="text-base font-bold metric-value text-gane-cyan">{totalFeatures}</div>
            <div className="text-[8px] text-muted-foreground uppercase tracking-wider">Features</div>
          </div>
          <div className="feature-card !p-2.5 text-center">
            <div className="text-base font-bold metric-value text-gane-green">{Object.keys(layerGroups).length}</div>
            <div className="text-[8px] text-muted-foreground uppercase tracking-wider">Layers</div>
          </div>
          <div className="feature-card !p-2.5 text-center">
            <div className="text-base font-bold metric-value text-gane-amber">{totalSections}</div>
            <div className="text-[8px] text-muted-foreground uppercase tracking-wider">G.A.N.E Subs</div>
          </div>
        </div>
      </div>

      {/* G.A.N.E Sections */}
      {activeTab === 'gane' && (
        <div className="px-4 pb-6 space-y-3">
          {Object.entries(layerGroups).map(([layer, sections]) => (
            <div key={layer}>
              <div className="flex items-center gap-2 mb-2">
                <Layers size={10} className="text-gane-violet" />
                <span className="text-[10px] font-semibold tracking-wider uppercase text-muted-foreground" style={{ fontFamily: 'Syne, sans-serif' }} dir="rtl">
                  {layer}
                </span>
                <div className="flex-1 h-px" style={{ background: 'oklch(1 0 0 / 5%)' }} />
              </div>
              {sections.map((section) => {
                const isExpanded = expandedId === section.id;
                return (
                  <div key={section.id} className="mb-2">
                    <button
                      onClick={() => setExpandedId(isExpanded ? null : section.id)}
                      className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all text-left ${
                        isExpanded ? 'orbital-panel-glow' : 'orbital-panel hover:border-white/8'
                      }`}
                    >
                      <span className={`text-xs w-6 ${colorMap[section.color]}`} style={{ fontFamily: 'JetBrains Mono, monospace' }}>
                        {String(section.number).padStart(2, '0')}
                      </span>
                      <div className="flex-1 min-w-0">
                        <div className="text-sm font-medium truncate">{section.titleEn}</div>
                        <div className="text-[10px] text-muted-foreground truncate" dir="rtl">{section.titleHe}</div>
                      </div>
                      <span className={`text-[9px] ${colorMap[section.color]}`} style={{ fontFamily: 'JetBrains Mono, monospace' }}>
                        {section.features.length}
                      </span>
                      {isExpanded
                        ? <ChevronDown size={14} className={colorMap[section.color]} />
                        : <ChevronRight size={14} className="text-muted-foreground" />
                      }
                    </button>

                    <AnimatePresence>
                      {isExpanded && (
                        <motion.div
                          initial={{ height: 0, opacity: 0 }}
                          animate={{ height: 'auto', opacity: 1 }}
                          exit={{ height: 0, opacity: 0 }}
                          transition={{ duration: 0.3, ease: [0.16, 1, 0.3, 1] }}
                          className="overflow-hidden"
                        >
                          <div className="mt-2 ml-4 pl-4 space-y-3" style={{ borderLeft: '1px solid oklch(1 0 0 / 5%)' }}>
                            {/* Hebrew Description */}
                            <div className="p-3 rounded-lg" style={{ background: 'oklch(1 0 0 / 3%)' }} dir="rtl">
                              <p className="text-xs leading-relaxed" style={{ color: 'oklch(0.80 0.005 210)' }}>{section.descriptionHe}</p>
                            </div>

                            {/* Features */}
                            <div className="text-[10px] font-semibold tracking-wider uppercase text-muted-foreground" style={{ fontFamily: 'Syne, sans-serif' }}>
                              Features ({section.features.length})
                            </div>
                            <div className="space-y-1.5">
                              {section.features.map((feature, fi) => (
                                <div
                                  key={fi}
                                  className={`flex items-center gap-2 px-3 py-2 rounded-lg ${bgMap[section.color]} border ${borderMap[section.color]}`}
                                >
                                  <CircleDot size={6} className={colorMap[section.color]} />
                                  <span className="text-xs flex-1">{feature.nameEn}</span>
                                  <span className="text-[10px] text-muted-foreground" dir="rtl">{feature.nameHe}</span>
                                </div>
                              ))}
                            </div>
                          </div>
                        </motion.div>
                      )}
                    </AnimatePresence>
                  </div>
                );
              })}
            </div>
          ))}
        </div>
      )}

      {/* G.A.N.E Parts */}
      {activeTab === 'gane' && (
        <div className="px-4 pb-6 space-y-2">
          {filteredSpec.map((part) => {
            const isExpanded = expandedId === part.id;
            const partColor = colorMap[part.color] || 'text-gane-cyan';
            return (
              <div key={part.id}>
                <button
                  onClick={() => setExpandedId(isExpanded ? null : part.id)}
                  className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all text-left ${
                    isExpanded ? 'orbital-panel-glow' : 'orbital-panel hover:border-white/8'
                  }`}
                >
                  <span className={`text-xs w-8 ${partColor}`} style={{ fontFamily: 'JetBrains Mono, monospace' }}>
                    {part.romanNumeral}
                  </span>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-medium truncate">{part.title}</div>
                    <div className="text-[10px] text-muted-foreground truncate">{part.description}</div>
                  </div>
                  <span className={`text-[9px] ${partColor}`} style={{ fontFamily: 'JetBrains Mono, monospace' }}>
                    {part.sections.length}
                  </span>
                  {isExpanded
                    ? <ChevronDown size={14} className="text-gane-cyan" />
                    : <ChevronRight size={14} className="text-muted-foreground" />
                  }
                </button>

                <AnimatePresence>
                  {isExpanded && (
                    <motion.div
                      initial={{ height: 0, opacity: 0 }}
                      animate={{ height: 'auto', opacity: 1 }}
                      exit={{ height: 0, opacity: 0 }}
                      transition={{ duration: 0.3, ease: [0.16, 1, 0.3, 1] }}
                      className="overflow-hidden"
                    >
                      <div className="mt-2 ml-4 pl-4 space-y-2" style={{ borderLeft: '1px solid oklch(0.82 0.15 192 / 10%)' }}>
                        {part.sections.map((sec) => (
                          <div key={sec.id} className="p-3 rounded-lg" style={{ background: 'oklch(1 0 0 / 3%)' }}>
                            <div className="flex items-center gap-2 mb-2">
                              <span className="text-[10px] text-gane-cyan" style={{ fontFamily: 'JetBrains Mono, monospace' }}>{sec.number}</span>
                              <span className="text-xs font-medium">{sec.title}</span>
                            </div>
                            <p className="text-[11px] leading-relaxed text-muted-foreground">
                              {sec.content.substring(0, 300)}
                              {sec.content.length > 300 && <span className="text-gane-cyan"> ...</span>}
                            </p>
                            {sec.subsections && sec.subsections.length > 0 && (
                              <div className="mt-2 space-y-1">
                                {sec.subsections.slice(0, 5).map((sub) => (
                                  <div key={sub.id} className="flex items-center gap-2 text-[10px] text-muted-foreground pl-2" style={{ borderLeft: '1px solid oklch(1 0 0 / 5%)' }}>
                                    <span className="text-gane-cyan" style={{ fontFamily: 'JetBrains Mono, monospace' }}>{sub.number}</span>
                                    <span>{sub.title}</span>
                                  </div>
                                ))}
                                {sec.subsections.length > 5 && (
                                  <div className="text-[9px] text-gane-cyan pl-2">+{sec.subsections.length - 5} more subsections</div>
                                )}
                              </div>
                            )}
                          </div>
                        ))}
                      </div>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            );
          })}
        </div>
      )}

      {/* Footer */}
      <div className="px-4 py-4" style={{ borderTop: '1px solid oklch(1 0 0 / 5%)' }}>
        <div className="text-center text-[10px] text-muted-foreground">
          G.A.N.E Engineering Master Specification v1.0
        </div>
        <div className="text-center text-[9px] text-muted-foreground mt-1">
          {totalFeatures} features across {ganeSections.length + specParts.length} documents
        </div>
      </div>
    </motion.div>
  );
}
