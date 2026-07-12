/**
 * G.A.N.E — Search Panel v3.0
 * Holographic search with animated results, smart categories, and ambient intelligence
 */
import { useState, useRef, useCallback, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Search, X, MapPin, Clock, Star, Navigation, Building2,
  Coffee, Fuel, ShoppingBag, Utensils, Hospital, Compass,
  Sparkles, TrendingUp, Zap, Globe, Hexagon
} from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { useLanguage } from "@/contexts/LanguageContext";
import { nearbySearch } from "@/lib/mapCompat";
import type { Place } from "@/lib/navStore";
import type { TranslationKey } from "@/lib/i18n";

const categories: { id: string; icon: typeof Utensils; i18nKey: TranslationKey; color: string; bg: string }[] = [
  { id: 'restaurant', icon: Utensils, i18nKey: 'search.food', color: '#ffaa00', bg: 'rgba(255,170,0,0.08)' },
  { id: 'gas_station', icon: Fuel, i18nKey: 'search.gas', color: '#00e88f', bg: 'rgba(0,232,143,0.08)' },
  { id: 'cafe', icon: Coffee, i18nKey: 'search.coffee', color: '#ff8800', bg: 'rgba(255,136,0,0.08)' },
  { id: 'parking', icon: Hexagon, i18nKey: 'search.parking', color: '#2563EB', bg: 'rgba(0,212,255,0.08)' },
  { id: 'shopping_mall', icon: ShoppingBag, i18nKey: 'search.shopping', color: '#8855ff', bg: 'rgba(136,85,255,0.08)' },
  { id: 'hospital', icon: Hospital, i18nKey: 'search.hospital', color: '#ff4466', bg: 'rgba(255,68,102,0.08)' },
];

export default function SearchPanel() {
  const { state, dispatch, mapRef, geocoderRef, placesServiceRef } = useNavigation();
  const { t, dir } = useLanguage();
  const [focused, setFocused] = useState(false);
  const [predictions, setPredictions] = useState<google.maps.places.AutocompletePrediction[]>([]);
  const [activeCategory, setActiveCategory] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const autocompleteRef = useRef<any>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  useEffect(() => {
    const checkGoogle = () => {
      if (window.google?.maps?.places?.AutocompleteService) {
        try { autocompleteRef.current = new (google.maps.places as any).AutocompleteService(); } catch { /* retry */ }
      }
    };
    checkGoogle();
    const timer = setInterval(checkGoogle, 500);
    return () => clearInterval(timer);
  }, []);

  const searchPlaces = useCallback((query: string) => {
    if (!query.trim() || !autocompleteRef.current) { setPredictions([]); return; }
    const request: google.maps.places.AutocompletionRequest = {
      input: query,
      locationBias: state.userLocation ? { center: state.userLocation, radius: 50000 } as unknown as google.maps.places.LocationBias : undefined };
    autocompleteRef.current.getPlacePredictions(request, (results: any, status: any) => {
      if (status === google.maps.places.PlacesServiceStatus.OK && results) setPredictions(results);
      else setPredictions([]);
    });
  }, [state.userLocation]);

  const handleInputChange = (value: string) => {
    dispatch({ type: 'SET_SEARCH_QUERY', query: value });
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => searchPlaces(value), 300);
  };

  const selectPrediction = useCallback((prediction: google.maps.places.AutocompletePrediction) => {
    if (!geocoderRef.current) return;
    dispatch({ type: 'SET_LOADING', loading: true });
    geocoderRef.current.geocode({ placeId: prediction.place_id }, (results, status) => {
      if (status === "OK" && results?.[0]) {
        const loc = results[0].geometry.location;
        const place: Place = {
          id: prediction.place_id,
          name: prediction.structured_formatting.main_text,
          address: prediction.structured_formatting.secondary_text || prediction.description,
          location: { lat: loc.lat(), lng: loc.lng() } };
        dispatch({ type: 'SET_DESTINATION', place });
        dispatch({ type: 'ADD_RECENT_SEARCH', place });
        dispatch({ type: 'SET_VIEW', view: 'route-plan' });
        dispatch({ type: 'SET_SEARCH_QUERY', query: '' });
        dispatch({ type: 'SET_LOADING', loading: false });
        setPredictions([]);
        mapRef.current?.panTo(place.location);
        mapRef.current?.setZoom(15);
      } else {
        dispatch({ type: 'SET_LOADING', loading: false });
      }
    });
  }, [dispatch, geocoderRef, mapRef]);

  const selectPlace = useCallback((place: Place) => {
    dispatch({ type: 'SET_DESTINATION', place });
    dispatch({ type: 'ADD_RECENT_SEARCH', place });
    dispatch({ type: 'SET_VIEW', view: 'route-plan' });
    mapRef.current?.panTo(place.location);
    mapRef.current?.setZoom(15);
  }, [dispatch, mapRef]);

  const searchCategory = useCallback((type: string) => {
    if (!placesServiceRef.current || !state.userLocation) return;
    setActiveCategory(type);
    dispatch({ type: 'SET_LOADING', loading: true });
    nearbySearch({
      placesService: placesServiceRef.current,
      location: state.userLocation,
      radius: 5000,
      type,
    }).then(({ places }) => {
      if (places.length > 0) {
        dispatch({ type: 'SET_SEARCH_RESULTS', results: places });
      } else {
        dispatch({ type: 'SET_LOADING', loading: false });
      }
    });
  }, [placesServiceRef, state.userLocation, dispatch]);

  const close = () => {
    dispatch({ type: 'SET_VIEW', view: 'map' });
    dispatch({ type: 'SET_SEARCH_QUERY', query: '' });
    setPredictions([]);
    setActiveCategory(null);
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.25 }}
      className="fixed inset-0 z-40 flex flex-col"
      style={{ background: 'rgba(2,4,8,0.98)' }}
    >
      {/* ── Search Header ── */}
      <motion.div
        initial={{ y: -20, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ delay: 0.05, type: 'spring', damping: 24 }}
        className="px-4 pt-4 pb-3"
      >
        <div className="flex items-center gap-3">
          <motion.button
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.85 }}
            onClick={close}
            className="w-10 h-10 rounded-xl flex items-center justify-center cursor-pointer transition-colors"
            style={{ background: 'rgba(229,231,235,0.4)', border: '1px solid rgba(229,231,235,0.6)' }}
            aria-label="Close search"
          >
            <X className="w-5 h-5 text-white/40" />
          </motion.button>

          <div className="flex-1 relative">
            {/* Search icon with pulse when focused */}
            <motion.div
              className="absolute left-3.5 top-1/2 -translate-y-1/2 z-10"
              animate={focused ? { scale: [1, 1.15, 1] } : {}}
              transition={{ duration: 1.5, repeat: Infinity }}
            >
              <Search className="w-4 h-4" style={{ color: focused ? '#2563EB' : 'rgba(156,163,175,0.6)' }} />
            </motion.div>

            <input
              ref={inputRef}
              type="text"
              value={state.searchQuery}
              onChange={e => handleInputChange(e.target.value)}
              onFocus={() => setFocused(true)}
              onBlur={() => setFocused(false)}
              placeholder={t('search.whereTo')}
              className="w-full py-3.5 pl-11 pr-4 rounded-xl text-sm text-white/90 placeholder:text-white/18 outline-none transition-all duration-300"
              style={{
                background: focused ? 'rgba(0,212,255,0.06)' : 'rgba(229,231,235,0.4)',
                border: focused ? '1px solid rgba(0,212,255,0.3)' : '1px solid rgba(229,231,235,0.6)',
                boxShadow: focused ? '0 0 24px rgba(0,212,255,0.08)' : 'none',
                direction: dir }}
              autoFocus
              aria-label="Search for a place"
            />

            {/* Scanning line when focused */}
            {focused && (
              <motion.div
                className="absolute bottom-0 left-0 right-0 h-px pointer-events-none"
                style={{ background: 'linear-gradient(90deg, transparent, #2563EB, transparent)' }}
                animate={{ opacity: [0.3, 0.7, 0.3] }}
                transition={{ duration: 2, repeat: Infinity }}
              />
            )}
          </div>
        </div>
      </motion.div>

      {/* ── Category Cards ── */}
      {!state.searchQuery && (
        <motion.div
          initial={{ y: 10, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ delay: 0.1 }}
          className="flex gap-2 px-4 pb-4 overflow-x-auto"
          style={{ scrollbarWidth: 'none' }}
        >
          {categories.map((cat, i) => (
            <motion.button
              key={cat.id}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.05 + 0.15 }}
              whileHover={{ scale: 1.04, y: -2 }}
              whileTap={{ scale: 0.95 }}
              onClick={() => searchCategory(cat.id)}
              className="flex flex-col items-center gap-1.5 px-4 py-3 rounded-xl whitespace-nowrap transition-all duration-300 cursor-pointer flex-shrink-0 relative overflow-hidden"
              style={{
                background: activeCategory === cat.id ? cat.bg : 'rgba(243,244,246,0.4)',
                border: activeCategory === cat.id ? `1px solid ${cat.color}40` : '1px solid rgba(229,231,235,0.4)',
                boxShadow: activeCategory === cat.id ? `0 0 16px ${cat.color}15` : 'none' }}
              aria-label={`Search for ${cat.id}`}
            >
              <cat.icon className="w-5 h-5" style={{ color: cat.color }} />
              <span className="text-[10px] font-medium" style={{ color: activeCategory === cat.id ? cat.color : 'rgba(107,114,128,0.7)' }}>
                {t(cat.i18nKey)}
              </span>
            </motion.button>
          ))}
        </motion.div>
      )}

      {/* ── Results Area ── */}
      <div className="flex-1 overflow-y-auto px-4" style={{ scrollbarWidth: 'none' }}>
        {/* Loading indicator */}
        {state.isLoading && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="flex items-center justify-center py-8"
          >
            <motion.div
              className="w-8 h-8 rounded-full border-2 border-transparent"
              style={{ borderTopColor: '#2563EB', borderRightColor: '#2563EB50' }}
              animate={{ rotate: 360 }}
              transition={{ duration: 0.8, repeat: Infinity, ease: 'linear' }}
            />
          </motion.div>
        )}

        {/* Autocomplete predictions */}
        <AnimatePresence mode="wait">
          {predictions.length > 0 && (
            <motion.div
              key="predictions"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="mb-4"
            >
              <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
                style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif' }}>
                <Sparkles className="w-3 h-3" style={{ color: '#2563EB' }} />
                {t('search.results')}
              </div>
              {predictions.map((pred, i) => (
                <motion.button
                  key={pred.place_id}
                  initial={{ opacity: 0, x: -12 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.04 }}
                  onClick={() => selectPrediction(pred)}
                  className="w-full flex items-start gap-3 px-3 py-3 rounded-xl transition-all duration-200 text-left group cursor-pointer"
                  style={{ direction: dir }}
                  onMouseEnter={e => {
                    e.currentTarget.style.background = 'rgba(0,212,255,0.04)';
                    e.currentTarget.style.border = '1px solid rgba(0,212,255,0.1)';
                  }}
                  onMouseLeave={e => {
                    e.currentTarget.style.background = 'transparent';
                    e.currentTarget.style.border = '1px solid transparent';
                  }}
                >
                  <div className="w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 mt-0.5"
                    style={{ background: 'rgba(0,212,255,0.08)' }}>
                    <MapPin className="w-4 h-4" style={{ color: '#2563EB' }} />
                  </div>
                  <div className="min-w-0 flex-1">
                    <div className="text-sm text-white/80 font-medium truncate">{pred.structured_formatting.main_text}</div>
                    <div className="text-xs text-white/25 truncate mt-0.5">{pred.structured_formatting.secondary_text}</div>
                  </div>
                  <Navigation className="w-4 h-4 text-white/0 group-hover:text-white/20 transition-all mt-1.5 flex-shrink-0" />
                </motion.button>
              ))}
            </motion.div>
          )}
        </AnimatePresence>

        {/* Category search results */}
        {state.searchResults.length > 0 && !state.searchQuery && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="mb-4"
          >
            <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
              style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif' }}>
              <Globe className="w-3 h-3" style={{ color: '#00e88f' }} />
              {t('search.nearby')}
            </div>
            {state.searchResults.map((place, i) => (
              <motion.button
                key={place.id}
                initial={{ opacity: 0, x: -12 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.04 }}
                onClick={() => selectPlace(place)}
                className="w-full flex items-start gap-3 px-3 py-3 rounded-xl transition-all duration-200 text-left group cursor-pointer"
                style={{ direction: dir }}
                onMouseEnter={e => {
                  e.currentTarget.style.background = 'rgba(0,232,143,0.04)';
                }}
                onMouseLeave={e => {
                  e.currentTarget.style.background = 'transparent';
                }}
              >
                <div className="w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 mt-0.5"
                  style={{ background: 'rgba(0,232,143,0.08)' }}>
                  <Building2 className="w-4 h-4" style={{ color: '#00e88f' }} />
                </div>
                <div className="min-w-0 flex-1">
                  <div className="text-sm text-white/80 font-medium truncate">{place.name}</div>
                  <div className="text-xs text-white/25 truncate mt-0.5">{place.address}</div>
                </div>
              </motion.button>
            ))}
          </motion.div>
        )}

        {/* Favorites */}
        {!state.searchQuery && predictions.length === 0 && (
          <>
            {state.favorites.length > 0 && (
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.15 }}
                className="mb-6"
              >
                <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
                  style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif' }}>
                  <Star className="w-3 h-3" style={{ color: '#ffaa00' }} />
                  {t('search.favorites')}
                </div>
                <div className="grid grid-cols-2 gap-2">
                  {state.favorites.map((fav, i) => (
                    <motion.button
                      key={fav.id}
                      initial={{ opacity: 0, scale: 0.95 }}
                      animate={{ opacity: 1, scale: 1 }}
                      transition={{ delay: i * 0.06 + 0.2 }}
                      whileHover={{ scale: 1.02, y: -2 }}
                      whileTap={{ scale: 0.97 }}
                      onClick={() => selectPlace(fav)}
                      className="flex items-center gap-3 px-3 py-3 rounded-xl transition-all duration-200 text-left cursor-pointer"
                      style={{
                        background: 'rgba(243,244,246,0.4)',
                        border: '1px solid rgba(229,231,235,0.4)',
                        direction: dir }}
                    >
                      <span className="text-xl">{fav.icon || '⭐'}</span>
                      <div className="min-w-0 flex-1">
                        <div className="text-sm text-white/70 font-medium truncate">{fav.name}</div>
                        <div className="text-[10px] text-white/20 truncate">{fav.address}</div>
                      </div>
                    </motion.button>
                  ))}
                </div>
              </motion.div>
            )}

            {/* Recent */}
            {state.recentSearches.length > 0 && (
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.25 }}
                className="mb-6"
              >
                <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
                  style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif' }}>
                  <Clock className="w-3 h-3" style={{ color: '#8855ff' }} />
                  {t('search.recent')}
                </div>
                {state.recentSearches.map((place, i) => (
                  <motion.button
                    key={place.id}
                    initial={{ opacity: 0, x: -8 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: i * 0.04 + 0.3 }}
                    onClick={() => selectPlace(place)}
                    className="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl transition-all duration-200 text-left group cursor-pointer"
                    style={{ direction: dir }}
                    onMouseEnter={e => { e.currentTarget.style.background = 'rgba(136,85,255,0.04)'; }}
                    onMouseLeave={e => { e.currentTarget.style.background = 'transparent'; }}
                  >
                    <div className="w-7 h-7 rounded-lg flex items-center justify-center flex-shrink-0"
                      style={{ background: 'rgba(136,85,255,0.08)' }}>
                      <Clock className="w-3.5 h-3.5" style={{ color: '#8855ff' }} />
                    </div>
                    <div className="min-w-0 flex-1">
                      <div className="text-sm text-white/55 group-hover:text-white/75 transition-colors">{place.name}</div>
                      <div className="text-[10px] text-white/18 truncate">{place.address}</div>
                    </div>
                    <Navigation className="w-3.5 h-3.5 text-white/0 group-hover:text-white/20 transition-all flex-shrink-0" />
                  </motion.button>
                ))}
              </motion.div>
            )}

            {/* AI Suggestions */}
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.35 }}
              className="mb-6"
            >
              <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
                style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif' }}>
                <Zap className="w-3 h-3" style={{ color: '#2563EB' }} />
                {t('search.suggestions')}
              </div>
              <div className="rounded-xl p-3 space-y-2"
                style={{ background: 'rgba(0,212,255,0.03)', border: '1px solid rgba(0,212,255,0.06)' }}>
                {[
                  { i18nKey: 'search.bestTime' as TranslationKey, icon: TrendingUp, color: '#00e88f' },
                  { i18nKey: 'search.parkingAvailable' as TranslationKey, icon: Compass, color: '#2563EB' },
                  { i18nKey: 'search.goodWeather' as TranslationKey, icon: Sparkles, color: '#ffaa00' },
                ].map((tip, i) => (
                  <motion.div
                    key={i}
                    initial={{ opacity: 0, x: -8 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: i * 0.08 + 0.4 }}
                    className="flex items-center gap-2.5 py-1"
                    style={{ direction: dir }}
                  >
                    <tip.icon className="w-3.5 h-3.5 flex-shrink-0" style={{ color: tip.color }} />
                    <span className="text-xs text-white/30">{t(tip.i18nKey)}</span>
                  </motion.div>
                ))}
              </div>
            </motion.div>
          </>
        )}
      </div>
    </motion.div>
  );
}
