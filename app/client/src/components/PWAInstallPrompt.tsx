/**
 * G.A.N.E — PWA Install Prompt
 * 
 * Shows a native-looking install banner when the app is installable.
 * Handles beforeinstallprompt event and provides install button.
 * Auto-dismisses after install or user dismissal.
 */
import { useState, useEffect, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Download, X, Smartphone } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";

interface BeforeInstallPromptEvent extends Event {
  prompt(): Promise<void>;
  userChoice: Promise<{ outcome: 'accepted' | 'dismissed' }>;
}

export default function PWAInstallPrompt() {
  const [deferredPrompt, setDeferredPrompt] = useState<BeforeInstallPromptEvent | null>(null);
  const [showBanner, setShowBanner] = useState(false);
  const [isInstalled, setIsInstalled] = useState(false);
  const { lang } = useLanguage();

  useEffect(() => {
    // Check if already installed
    if (window.matchMedia('(display-mode: standalone)').matches || (window.navigator as any).standalone) {
      setIsInstalled(true);
      return;
    }

    // Check if user dismissed before (respect for 7 days)
    const dismissed = localStorage.getItem('gane-pwa-dismissed');
    if (dismissed) {
      const dismissedAt = parseInt(dismissed, 10);
      if (Date.now() - dismissedAt < 7 * 24 * 60 * 60 * 1000) return;
    }

    const handler = (e: Event) => {
      e.preventDefault();
      setDeferredPrompt(e as BeforeInstallPromptEvent);
      // Show banner after a short delay so the user sees the app first
      setTimeout(() => setShowBanner(true), 3000);
    };

    window.addEventListener('beforeinstallprompt', handler);
    window.addEventListener('appinstalled', () => {
      setIsInstalled(true);
      setShowBanner(false);
      setDeferredPrompt(null);
    });

    return () => {
      window.removeEventListener('beforeinstallprompt', handler);
    };
  }, []);

  const handleInstall = useCallback(async () => {
    if (!deferredPrompt) return;
    await deferredPrompt.prompt();
    const { outcome } = await deferredPrompt.userChoice;
    if (outcome === 'accepted') {
      setIsInstalled(true);
    }
    setShowBanner(false);
    setDeferredPrompt(null);
  }, [deferredPrompt]);

  const handleDismiss = useCallback(() => {
    setShowBanner(false);
    localStorage.setItem('gane-pwa-dismissed', Date.now().toString());
  }, []);

  if (isInstalled) return null;

  const texts = {
    en: { title: 'Install G.A.N.E NAV', desc: 'Add to home screen for the best experience', install: 'Install' },
    he: { title: 'התקן G.A.N.E NAV', desc: 'הוסף למסך הבית לחוויה הטובה ביותר', install: 'התקן' },
    ar: { title: 'تثبيت G.A.N.E NAV', desc: 'أضف إلى الشاشة الرئيسية للحصول على أفضل تجربة', install: 'تثبيت' },
    es: { title: 'Instalar G.A.N.E NAV', desc: 'Agregar a pantalla de inicio', install: 'Instalar' },
    fr: { title: 'Installer G.A.N.E NAV', desc: 'Ajouter à l\'écran d\'accueil', install: 'Installer' },
    de: { title: 'G.A.N.E NAV installieren', desc: 'Zum Startbildschirm hinzufügen', install: 'Installieren' },
    ru: { title: 'Установить G.A.N.E NAV', desc: 'Добавить на главный экран', install: 'Установить' },
    zh: { title: '安装 G.A.N.E NAV', desc: '添加到主屏幕以获得最佳体验', install: '安装' },
  };
  const t = (texts as any)[lang] || texts.en;

  return (
    <AnimatePresence>
      {showBanner && (
        <motion.div
          initial={{ y: 100, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: 100, opacity: 0 }}
          transition={{ type: 'spring', damping: 25, stiffness: 300 }}
          className="fixed bottom-20 left-4 right-4 z-50 sm:left-auto sm:right-6 sm:bottom-6 sm:w-[360px]"
        >
          <div className="rounded-2xl overflow-hidden"
            style={{
              background: '#FFFFFF',
              border: '1px solid #E5E7EB',
              boxShadow: '0 8px 32px rgba(0,0,0,0.12)',
            }}>
            <div className="p-4 flex items-start gap-3">
              <div className="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0"
                style={{ background: 'linear-gradient(135deg, #2563EB, #16A34A)', boxShadow: '0 2px 8px rgba(37,99,235,0.3)' }}>
                <Smartphone className="w-6 h-6 text-white" />
              </div>
              <div className="flex-1 min-w-0">
                <div className="text-sm font-bold text-gray-900">{t.title}</div>
                <div className="text-xs text-gray-500 mt-0.5">{t.desc}</div>
                <div className="flex items-center gap-2 mt-3">
                  <motion.button
                    whileTap={{ scale: 0.95 }}
                    onClick={handleInstall}
                    className="flex items-center gap-1.5 px-4 py-2 rounded-xl text-xs font-bold text-white cursor-pointer"
                    style={{ background: 'linear-gradient(135deg, #2563EB, #1D4ED8)', minHeight: '36px' }}
                  >
                    <Download className="w-3.5 h-3.5" />
                    {t.install}
                  </motion.button>
                  <button
                    onClick={handleDismiss}
                    className="px-3 py-2 rounded-xl text-xs font-medium text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors cursor-pointer"
                    style={{ minHeight: '36px' }}
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
              </div>
            </div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
