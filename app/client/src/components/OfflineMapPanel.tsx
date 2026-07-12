/**
 * G.A.N.E NAV — Offline Map Tiles Panel
 * =======================================
 * Full offline map management UI with region download, storage management,
 * offline routes, and delta updates.
 */
import { useState, useEffect, useCallback, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Download, Pause, Play, Trash2, HardDrive, Map, MapPin,
  Wifi, WifiOff, RefreshCw, Database, Layers, Globe,
  ChevronRight, Check, AlertTriangle, Compass, Route,
  Plus, X, Clock, ArrowDown, ArrowUp, Satellite, Mountain
} from 'lucide-react';
import { useLanguage } from '@/contexts/LanguageContext';
import {
  getOfflineMapEngine, PREDEFINED_REGIONS,
  type DownloadRegion, type TileProvider, type OfflineMapState
} from '@/engine/offlineMapEngine';

const PROVIDER_INFO: Record<TileProvider, { label: string; icon: typeof Map; color: string }> = {
  osm: { label: 'OpenStreetMap', icon: Map, color: '#7eb356' },
  satellite: { label: 'Satellite', icon: Satellite, color: '#4488ff' },
  terrain: { label: 'Terrain', icon: Mountain, color: '#cc8844' },
  topo: { label: 'Topographic', icon: Compass, color: '#66aa88' },
  dark: { label: 'Dark Mode', icon: Globe, color: '#8866cc' },
  hybrid: { label: 'Hybrid', icon: Layers, color: '#44aacc' },
};

function StorageBar({ used, total }: { used: number; total: number }) {
  const pct = total > 0 ? Math.min((used / total) * 100, 100) : 0;
  const color = pct > 90 ? '#ff3355' : pct > 70 ? '#ff9900' : '#00cc66';
  return (
    <div className="w-full">
      <div className="flex justify-between text-xs mb-1" style={{ color: 'rgba(0,0,0,0.5)' }}>
        <span>{formatBytes(used)} used</span>
        <span>{formatBytes(total)} total</span>
      </div>
      <div className="h-2 rounded-full overflow-hidden" style={{ background: 'rgba(0,0,0,0.06)' }}>
        <motion.div
          className="h-full rounded-full"
          style={{ background: color }}
          initial={{ width: 0 }}
          animate={{ width: `${pct}%` }}
          transition={{ duration: 0.5 }}
        />
      </div>
    </div>
  );
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

function RegionCard({
  region,
  onDownload,
  onPause,
  onDelete,
  isActive,
}: {
  region: DownloadRegion;
  onDownload: () => void;
  onPause: () => void;
  onDelete: () => void;
  isActive: boolean;
}) {
  const statusColors: Record<string, string> = {
    pending: '#8899aa',
    downloading: '#4488ff',
    paused: '#ff9900',
    complete: '#00cc66',
    error: '#ff3355',
  };

  const statusLabels: Record<string, string> = {
    pending: 'Ready',
    downloading: 'Downloading...',
    paused: 'Paused',
    complete: 'Complete',
    error: 'Error',
  };

  const progress = region.totalTiles > 0
    ? Math.round((region.downloadedTiles / region.totalTiles) * 100)
    : 0;

  return (
    <motion.div
      className="rounded-xl p-4 mb-3"
      style={{
        background: 'rgba(255,255,255,0.7)',
        border: `1px solid ${isActive ? '#4488ff' : 'rgba(0,0,0,0.06)'}`,
        boxShadow: isActive ? '0 0 20px rgba(68,136,255,0.15)' : '0 2px 8px rgba(0,0,0,0.04)',
      }}
      layout
    >
      <div className="flex items-start justify-between mb-2">
        <div>
          <div className="font-semibold text-sm" style={{ color: '#1a1a2e' }}>{region.name}</div>
          <div className="text-xs mt-0.5" style={{ color: 'rgba(0,0,0,0.4)' }}>
            {region.totalTiles.toLocaleString()} tiles · {region.providers.join(', ')}
          </div>
        </div>
        <div className="flex items-center gap-1.5">
          <span
            className="text-[10px] font-medium px-2 py-0.5 rounded-full"
            style={{
              background: `${statusColors[region.status]}15`,
              color: statusColors[region.status],
            }}
          >
            {statusLabels[region.status]}
          </span>
        </div>
      </div>

      {/* Progress bar */}
      {(region.status === 'downloading' || region.status === 'paused' || region.status === 'complete') && (
        <div className="mb-2">
          <div className="h-1.5 rounded-full overflow-hidden" style={{ background: 'rgba(0,0,0,0.06)' }}>
            <motion.div
              className="h-full rounded-full"
              style={{ background: statusColors[region.status] }}
              animate={{ width: `${progress}%` }}
              transition={{ duration: 0.3 }}
            />
          </div>
          <div className="flex justify-between text-[10px] mt-1" style={{ color: 'rgba(0,0,0,0.4)' }}>
            <span>{progress}%</span>
            <span>{formatBytes(region.totalSize)}</span>
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-2 mt-2">
        {region.status === 'pending' && (
          <button
            onClick={onDownload}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all hover:scale-105"
            style={{ background: '#4488ff', color: '#fff' }}
          >
            <Download size={12} /> Download
          </button>
        )}
        {region.status === 'downloading' && (
          <button
            onClick={onPause}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all hover:scale-105"
            style={{ background: '#ff9900', color: '#fff' }}
          >
            <Pause size={12} /> Pause
          </button>
        )}
        {region.status === 'paused' && (
          <button
            onClick={onDownload}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all hover:scale-105"
            style={{ background: '#00cc66', color: '#fff' }}
          >
            <Play size={12} /> Resume
          </button>
        )}
        <button
          onClick={onDelete}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all hover:scale-105"
          style={{ background: 'rgba(255,51,85,0.1)', color: '#ff3355' }}
        >
          <Trash2 size={12} /> Delete
        </button>
      </div>
    </motion.div>
  );
}

export default function OfflineMapPanel({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const engine = useMemo(() => getOfflineMapEngine(), []);
  const [state, setState] = useState<OfflineMapState>(engine.getState());
  const [activeTab, setActiveTab] = useState<'regions' | 'routes' | 'storage'>('regions');
  const [showAddRegion, setShowAddRegion] = useState(false);
  const [customRegion, setCustomRegion] = useState({
    name: '',
    north: 33.4, south: 29.4, east: 35.9, west: 34.2,
    minZoom: 8, maxZoom: 15,
    providers: ['osm'] as TileProvider[],
  });

  useEffect(() => {
    engine.init();
    const unsub = engine.subscribe(setState);
    return () => { unsub(); };
  }, [engine]);

  const handleAddPredefined = useCallback(async (idx: number) => {
    const preset = PREDEFINED_REGIONS[idx];
    if (!preset) return;
    await engine.createRegion(
      preset.name,
      preset.bounds,
      preset.minZoom,
      preset.maxZoom,
      preset.providers
    );
  }, [engine]);

  const handleAddCustom = useCallback(async () => {
    if (!customRegion.name) return;
    await engine.createRegion(
      customRegion.name,
      { north: customRegion.north, south: customRegion.south, east: customRegion.east, west: customRegion.west },
      customRegion.minZoom,
      customRegion.maxZoom,
      customRegion.providers
    );
    setShowAddRegion(false);
  }, [engine, customRegion]);

  const tabs = [
    { id: 'regions' as const, label: 'Regions', icon: Map },
    { id: 'routes' as const, label: 'Routes', icon: Route },
    { id: 'storage' as const, label: 'Storage', icon: HardDrive },
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
            style={{ background: 'linear-gradient(135deg, #4488ff, #00cc66)' }}>
            <Download size={20} color="#fff" />
          </div>
          <div>
            <h2 className="text-base font-bold" style={{ color: '#1a1a2e' }}>Offline Maps</h2>
            <div className="flex items-center gap-2 text-xs" style={{ color: 'rgba(0,0,0,0.5)' }}>
              {state.isOnline ? (
                <><Wifi size={11} className="text-green-500" /> Online</>
              ) : (
                <><WifiOff size={11} className="text-red-500" /> Offline</>
              )}
              <span>·</span>
              <span>{state.regions.filter(r => r.status === 'complete').length} regions cached</span>
            </div>
          </div>
        </div>

        {/* Storage overview */}
        <StorageBar
          used={state.storageStats.totalSize}
          total={state.storageStats.quotaSize}
        />

        {/* Download speed indicator */}
        {state.activeDownload && (
          <motion.div
            className="mt-2 flex items-center gap-2 text-xs"
            style={{ color: '#4488ff' }}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
          >
            <ArrowDown size={12} className="animate-bounce" />
            <span>{formatBytes(state.downloadSpeed)}/s</span>
            <span>· {state.downloadProgress}%</span>
          </motion.div>
        )}
      </div>

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
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-5 pb-5">
        <AnimatePresence mode="wait">
          {activeTab === 'regions' && (
            <motion.div key="regions" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
              {/* Add region button */}
              <button
                onClick={() => setShowAddRegion(!showAddRegion)}
                className="w-full flex items-center justify-center gap-2 py-2.5 rounded-xl mb-3 text-xs font-medium transition-all hover:scale-[1.02]"
                style={{
                  background: 'rgba(68,136,255,0.08)',
                  color: '#4488ff',
                  border: '1px dashed rgba(68,136,255,0.3)',
                }}
              >
                <Plus size={14} />
                Add Region
              </button>

              {/* Add region form */}
              <AnimatePresence>
                {showAddRegion && (
                  <motion.div
                    className="rounded-xl p-4 mb-3"
                    style={{ background: 'rgba(68,136,255,0.04)', border: '1px solid rgba(68,136,255,0.15)' }}
                    initial={{ height: 0, opacity: 0 }}
                    animate={{ height: 'auto', opacity: 1 }}
                    exit={{ height: 0, opacity: 0 }}
                  >
                    <div className="text-xs font-semibold mb-3" style={{ color: '#1a1a2e' }}>
                      Quick Add — Predefined Regions
                    </div>
                    <div className="grid grid-cols-2 gap-2 mb-4">
                      {PREDEFINED_REGIONS.map((preset, idx) => (
                        <button
                          key={idx}
                          onClick={() => handleAddPredefined(idx)}
                          className="text-left p-2.5 rounded-lg text-xs transition-all hover:scale-[1.02]"
                          style={{
                            background: 'rgba(255,255,255,0.8)',
                            border: '1px solid rgba(0,0,0,0.06)',
                          }}
                        >
                          <div className="font-medium" style={{ color: '#1a1a2e' }}>{preset.name}</div>
                          <div className="text-[10px] mt-0.5" style={{ color: 'rgba(0,0,0,0.4)' }}>
                            Z{preset.minZoom}-{preset.maxZoom} · {preset.providers.join(', ')}
                          </div>
                        </button>
                      ))}
                    </div>

                    <div className="text-xs font-semibold mb-2" style={{ color: '#1a1a2e' }}>
                      Custom Region
                    </div>
                    <input
                      type="text"
                      placeholder="Region name..."
                      value={customRegion.name}
                      onChange={e => setCustomRegion(p => ({ ...p, name: e.target.value }))}
                      className="w-full px-3 py-2 rounded-lg text-xs mb-2"
                      style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                    />
                    <div className="grid grid-cols-2 gap-2 mb-2">
                      {(['north', 'south', 'east', 'west'] as const).map(dir => (
                        <div key={dir}>
                          <label className="text-[10px] uppercase" style={{ color: 'rgba(0,0,0,0.4)' }}>{dir}</label>
                          <input
                            type="number"
                            step="0.1"
                            value={customRegion[dir]}
                            onChange={e => setCustomRegion(p => ({ ...p, [dir]: parseFloat(e.target.value) }))}
                            className="w-full px-2 py-1 rounded text-xs"
                            style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                          />
                        </div>
                      ))}
                    </div>
                    <div className="flex gap-2 mb-2">
                      <div>
                        <label className="text-[10px]" style={{ color: 'rgba(0,0,0,0.4)' }}>Min Zoom</label>
                        <input
                          type="number" min={1} max={18}
                          value={customRegion.minZoom}
                          onChange={e => setCustomRegion(p => ({ ...p, minZoom: parseInt(e.target.value) }))}
                          className="w-full px-2 py-1 rounded text-xs"
                          style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                        />
                      </div>
                      <div>
                        <label className="text-[10px]" style={{ color: 'rgba(0,0,0,0.4)' }}>Max Zoom</label>
                        <input
                          type="number" min={1} max={18}
                          value={customRegion.maxZoom}
                          onChange={e => setCustomRegion(p => ({ ...p, maxZoom: parseInt(e.target.value) }))}
                          className="w-full px-2 py-1 rounded text-xs"
                          style={{ background: 'rgba(255,255,255,0.8)', border: '1px solid rgba(0,0,0,0.1)', color: '#1a1a2e' }}
                        />
                      </div>
                    </div>
                    <div className="flex flex-wrap gap-1.5 mb-3">
                      {(Object.keys(PROVIDER_INFO) as TileProvider[]).map(p => (
                        <button
                          key={p}
                          onClick={() => setCustomRegion(prev => ({
                            ...prev,
                            providers: prev.providers.includes(p)
                              ? prev.providers.filter(x => x !== p)
                              : [...prev.providers, p],
                          }))}
                          className="px-2 py-1 rounded text-[10px] font-medium transition-all"
                          style={{
                            background: customRegion.providers.includes(p) ? PROVIDER_INFO[p].color : 'rgba(0,0,0,0.04)',
                            color: customRegion.providers.includes(p) ? '#fff' : 'rgba(0,0,0,0.5)',
                          }}
                        >
                          {PROVIDER_INFO[p].label}
                        </button>
                      ))}
                    </div>
                    <button
                      onClick={handleAddCustom}
                      className="w-full py-2 rounded-lg text-xs font-medium transition-all hover:scale-[1.02]"
                      style={{ background: '#4488ff', color: '#fff' }}
                    >
                      Create Region
                    </button>
                  </motion.div>
                )}
              </AnimatePresence>

              {/* Region list */}
              {state.regions.length === 0 ? (
                <div className="text-center py-8">
                  <Map size={32} className="mx-auto mb-2" style={{ color: 'rgba(0,0,0,0.15)' }} />
                  <div className="text-xs" style={{ color: 'rgba(0,0,0,0.4)' }}>
                    No regions downloaded yet
                  </div>
                  <div className="text-[10px] mt-1" style={{ color: 'rgba(0,0,0,0.3)' }}>
                    Add a region above to start caching map tiles
                  </div>
                </div>
              ) : (
                state.regions.map(region => (
                  <RegionCard
                    key={region.id}
                    region={region}
                    isActive={state.activeDownload === region.id}
                    onDownload={() => engine.startDownload(region.id)}
                    onPause={() => engine.pauseDownload()}
                    onDelete={() => engine.deleteRegion(region.id)}
                  />
                ))
              )}
            </motion.div>
          )}

          {activeTab === 'routes' && (
            <motion.div key="routes" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
              {state.offlineRoutes.length === 0 ? (
                <div className="text-center py-8">
                  <Route size={32} className="mx-auto mb-2" style={{ color: 'rgba(0,0,0,0.15)' }} />
                  <div className="text-xs" style={{ color: 'rgba(0,0,0,0.4)' }}>
                    No offline routes saved
                  </div>
                  <div className="text-[10px] mt-1" style={{ color: 'rgba(0,0,0,0.3)' }}>
                    Save routes while online for offline navigation
                  </div>
                </div>
              ) : (
                state.offlineRoutes.map(route => (
                  <div
                    key={route.id}
                    className="rounded-xl p-3 mb-2"
                    style={{ background: 'rgba(255,255,255,0.7)', border: '1px solid rgba(0,0,0,0.06)' }}
                  >
                    <div className="font-medium text-xs" style={{ color: '#1a1a2e' }}>{route.name}</div>
                    <div className="flex gap-3 mt-1 text-[10px]" style={{ color: 'rgba(0,0,0,0.4)' }}>
                      <span>{(route.distance / 1000).toFixed(1)} km</span>
                      <span>{Math.round(route.duration / 60)} min</span>
                      <span>{route.waypoints.length} waypoints</span>
                    </div>
                  </div>
                ))
              )}
            </motion.div>
          )}

          {activeTab === 'storage' && (
            <motion.div key="storage" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
              {/* Provider breakdown */}
              <div className="rounded-xl p-4 mb-3" style={{ background: 'rgba(255,255,255,0.7)', border: '1px solid rgba(0,0,0,0.06)' }}>
                <div className="text-xs font-semibold mb-3" style={{ color: '#1a1a2e' }}>Storage by Provider</div>
                {(Object.entries(state.storageStats.providerBreakdown) as [TileProvider, { count: number; size: number }][]).map(([provider, data]) => (
                  <div key={provider} className="flex items-center justify-between py-1.5">
                    <div className="flex items-center gap-2">
                      <div className="w-2 h-2 rounded-full" style={{ background: PROVIDER_INFO[provider]?.color ?? '#888' }} />
                      <span className="text-xs" style={{ color: '#1a1a2e' }}>{PROVIDER_INFO[provider]?.label ?? provider}</span>
                    </div>
                    <div className="text-xs" style={{ color: 'rgba(0,0,0,0.4)' }}>
                      {data.count.toLocaleString()} tiles · {formatBytes(data.size)}
                    </div>
                  </div>
                ))}
              </div>

              {/* Stats */}
              <div className="grid grid-cols-2 gap-2 mb-3">
                {[
                  { label: 'Total Tiles', value: state.storageStats.tileCount.toLocaleString(), icon: Layers },
                  { label: 'Regions', value: state.storageStats.regionCount.toString(), icon: Map },
                  { label: 'Routes', value: state.storageStats.routeCount.toString(), icon: Route },
                  { label: 'Delta Updates', value: state.pendingDeltaUpdates.toString(), icon: RefreshCw },
                ].map(stat => (
                  <div
                    key={stat.label}
                    className="rounded-xl p-3"
                    style={{ background: 'rgba(255,255,255,0.7)', border: '1px solid rgba(0,0,0,0.06)' }}
                  >
                    <stat.icon size={14} style={{ color: 'rgba(0,0,0,0.3)' }} />
                    <div className="text-lg font-bold mt-1" style={{ color: '#1a1a2e' }}>{stat.value}</div>
                    <div className="text-[10px]" style={{ color: 'rgba(0,0,0,0.4)' }}>{stat.label}</div>
                  </div>
                ))}
              </div>

              {/* Actions */}
              <div className="flex flex-col gap-2">
                <button
                  onClick={() => engine.checkForUpdates()}
                  className="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-medium transition-all hover:scale-[1.02]"
                  style={{ background: 'rgba(68,136,255,0.08)', color: '#4488ff' }}
                >
                  <RefreshCw size={14} /> Check for Updates
                </button>
                <button
                  onClick={() => engine.refreshStats()}
                  className="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-medium transition-all hover:scale-[1.02]"
                  style={{ background: 'rgba(0,204,102,0.08)', color: '#00cc66' }}
                >
                  <Database size={14} /> Refresh Stats
                </button>
                <button
                  onClick={() => engine.evictOldTiles(50)}
                  className="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-medium transition-all hover:scale-[1.02]"
                  style={{ background: 'rgba(255,153,0,0.08)', color: '#ff9900' }}
                >
                  <Trash2 size={14} /> Free 50MB (Evict Old)
                </button>
                <button
                  onClick={() => {
                    if (confirm('Delete ALL offline data? This cannot be undone.')) {
                      engine.clearAllData();
                    }
                  }}
                  className="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-medium transition-all hover:scale-[1.02]"
                  style={{ background: 'rgba(255,51,85,0.08)', color: '#ff3355' }}
                >
                  <AlertTriangle size={14} /> Clear All Data
                </button>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
