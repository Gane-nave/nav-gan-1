/**
 * G.A.N.E — Language Context
 * Manages current language, direction, and provides translation function
 * Auto-detects browser language on first visit
 */
import { createContext, useContext, useState, useCallback, useEffect, type ReactNode } from 'react';
import { t, getLanguage, languages, type TranslationKey, type Language } from '@/lib/i18n';

interface LanguageContextType {
  lang: string;
  language: Language;
  dir: 'ltr' | 'rtl';
  setLang: (code: string) => void;
  t: (key: TranslationKey) => string;
}

const LanguageContext = createContext<LanguageContextType | null>(null);

/** Detect the best matching language from browser settings */
function detectBrowserLanguage(): string {
  if (typeof window === 'undefined') return 'en';
  const supportedCodes = languages.map(l => l.code);
  // Check navigator.languages first (ordered by preference)
  const browserLangs = navigator.languages || [navigator.language];
  for (const bl of browserLangs) {
    const code = bl.split('-')[0].toLowerCase();
    if (supportedCodes.includes(code)) return code;
  }
  return 'en';
}

export function LanguageProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState(() => {
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('gane-lang');
      if (saved) return saved;
      // Auto-detect on first visit
      const detected = detectBrowserLanguage();
      localStorage.setItem('gane-lang', detected);
      return detected;
    }
    return 'en';
  });

  const language = getLanguage(lang);
  const dir = language.dir;

  const setLang = useCallback((code: string) => {
    setLangState(code);
    if (typeof window !== 'undefined') {
      localStorage.setItem('gane-lang', code);
    }
  }, []);

  useEffect(() => {
    document.documentElement.dir = dir;
    document.documentElement.lang = lang;
  }, [dir, lang]);

  const translate = useCallback((key: TranslationKey) => t(key, lang), [lang]);

  return (
    <LanguageContext.Provider value={{ lang, language, dir, setLang, t: translate }}>
      {children}
    </LanguageContext.Provider>
  );
}

export function useLanguage() {
  const ctx = useContext(LanguageContext);
  if (!ctx) throw new Error('useLanguage must be used within LanguageProvider');
  return ctx;
}
