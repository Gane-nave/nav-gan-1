/**
 * G.A.N.E i18n — Internationalization bundle
 * Usage in app:
 *   const t = GANE_I18N.get(userLang).t;
 *   element.textContent = t('mode.full');
 * Supports: English, Hebrew (RTL), Arabic (RTL)
 */
(function(global) {
'use strict';

const EN = {
  _meta: { name: 'English', dir: 'ltr', code: 'en' },
  app: {
    name: 'G.A.N.E Navigator',
    tagline: 'Global Autonomous Navigation Engine',
    searchPlaceholder: 'Where to? (address, coords, voice)'
  },
  mode: {
    FULL: 'Full positioning',
    DEGRADED: 'Degraded',
    DR_ONLY: 'Dead reckoning',
    LOST: 'Position lost',
    RECOVERY: 'Recovering'
  },
  env: {
    OPEN: 'Open sky', URBAN: 'Urban', CANYON: 'Urban canyon',
    TUNNEL: 'Tunnel', INDOOR: 'Indoors', UNKNOWN: 'Unknown'
  },
  phase: {
    HEALTHY: 'System healthy', CAUTIOUS: 'Minor anomalies',
    DEGRADED: 'Operating degraded', RESTRICTED: 'Restricted mode', FAIL_SAFE: 'Fail-safe active'
  },
  panels: {
    layers: 'Map Layers', gnss: 'GNSS Signals', ai: 'AI Copilot',
    diagnostics: 'Diagnostics', replay: 'Mission Log'
  },
  layers: {
    street: 'Street', dark: 'Dark', satellite: 'Satellite', terrain: 'Terrain',
    weather: 'Weather radar', transit: 'Transit', hiking: 'Hiking trails',
    cacheRegion: 'Cache current region'
  },
  diagnostics: {
    kpi: 'Key metrics', availability: 'Availability', continuity: 'Continuity',
    modeCycles: 'Mode cycles', integrityEvents: 'Integrity events',
    engineState: 'Engine state', currentMode: 'Current mode', ekfCycles: 'EKF cycles',
    uncertainty: 'Uncertainty', transitions: 'Transitions',
    watchdog: 'Watchdog', providers: 'Providers (circuit breaker)',
    runFull: 'Run full diagnostic'
  },
  emergency: {
    crashDetected: 'Crash detected',
    severityMinor: 'Minor impact', severityMajor: 'Major impact', severitySevere: 'Severe impact',
    beaconActive: 'Emergency beacon active',
    sosButton: 'SOS'
  },
  reality: {
    selfCorrecting: 'Self-correcting', cycle: 'Cycle',
    predictions: 'Predictions', contradictions: 'Contradictions',
    recurringFailures: 'Recurring failures', overridesFired: 'Overrides fired',
    etaCalibration: 'ETA calibration'
  },
  consciousness: {
    governing: 'Governing', phase: 'Phase', readiness: 'Readiness',
    selfConfidence: 'Self-confidence', verdict: 'Verdict', failsafe: 'Fail-safe',
    decisionsBlocked: 'Decisions blocked', auditEntries: 'Audit entries',
    trusting: 'Trusting', watchful: 'Watchful', uncertain: 'Uncertain', distrustful: 'Distrustful'
  },
  actions: {
    exportMission: 'Export JSON', replayMission: 'Replay', clearMission: 'Clear',
    centerOnMe: 'Center on me', openInMap: 'Open in map', share: 'Share'
  },
  errors: {
    gpsDenied: 'Location permission denied', gpsTimeout: 'GPS timeout',
    noResults: 'No results found', offline: 'You are offline',
    bundleMissing: 'Core bundle missing'
  },
  warnings: {
    lowConfidence: 'Low confidence — system uncertain',
    actionBlocked: 'Action blocked by safety system',
    accuracyExceeds: 'GPS accuracy exceeds usable limit',
    signalLow: 'Signal quality below threshold',
    satellitesFew: 'Too few satellites (need 4)'
  }
};

const HE = {
  _meta: { name: 'עברית', dir: 'rtl', code: 'he' },
  app: {
    name: 'G.A.N.E ניווט',
    tagline: 'מנוע ניווט אוטונומי גלובלי',
    searchPlaceholder: 'לאן? (כתובת, נ.צ., קול)'
  },
  mode: {
    FULL: 'מיקום מלא', DEGRADED: 'מצב מוגבל',
    DR_ONLY: 'ניווט בהשערה', LOST: 'מיקום אבד', RECOVERY: 'בהתאוששות'
  },
  env: {
    OPEN: 'שמיים פתוחים', URBAN: 'אזור עירוני', CANYON: 'קניון עירוני',
    TUNNEL: 'מנהרה', INDOOR: 'בתוך מבנה', UNKNOWN: 'לא ידוע'
  },
  phase: {
    HEALTHY: 'המערכת תקינה', CAUTIOUS: 'חריגות קלות',
    DEGRADED: 'פועלת במצב מוגבל', RESTRICTED: 'מצב מוגבל', FAIL_SAFE: 'מצב בטיחות פעיל'
  },
  panels: {
    layers: 'שכבות מפה', gnss: 'אותות GNSS', ai: 'עוזר AI',
    diagnostics: 'אבחון', replay: 'יומן משימה'
  },
  layers: {
    street: 'רחוב', dark: 'כהה', satellite: 'לוויין', terrain: 'טופוגרפי',
    weather: 'מכ״ם מזג אוויר', transit: 'תחבורה', hiking: 'שבילי הליכה',
    cacheRegion: 'שמור אזור במטמון'
  },
  diagnostics: {
    kpi: 'מדדים', availability: 'זמינות', continuity: 'רציפות',
    modeCycles: 'מחזורי מצב', integrityEvents: 'אירועי יושרה',
    engineState: 'מצב מנוע', currentMode: 'מצב נוכחי', ekfCycles: 'מחזורי EKF',
    uncertainty: 'אי-ודאות', transitions: 'מעברים',
    watchdog: 'שומר', providers: 'ספקים (מפסק זרם)',
    runFull: 'הרץ אבחון מלא'
  },
  emergency: {
    crashDetected: 'זוהתה תאונה',
    severityMinor: 'פגיעה קלה', severityMajor: 'פגיעה קשה', severitySevere: 'פגיעה חמורה',
    beaconActive: 'משואת חירום פעילה',
    sosButton: 'חירום'
  },
  reality: {
    selfCorrecting: 'מתקן את עצמו', cycle: 'מחזור',
    predictions: 'חיזויים', contradictions: 'סתירות',
    recurringFailures: 'כשלים חוזרים', overridesFired: 'עקיפות שהופעלו',
    etaCalibration: 'כיול זמן הגעה'
  },
  consciousness: {
    governing: 'מנהל', phase: 'שלב', readiness: 'מוכנות',
    selfConfidence: 'ביטחון עצמי', verdict: 'מסקנה', failsafe: 'בטיחות-כשל',
    decisionsBlocked: 'החלטות שנחסמו', auditEntries: 'רישומי ביקורת',
    trusting: 'סומך', watchful: 'מתריע', uncertain: 'לא בטוח', distrustful: 'לא סומך'
  },
  actions: {
    exportMission: 'ייצא JSON', replayMission: 'שחזור', clearMission: 'נקה',
    centerOnMe: 'מרכז עלי', openInMap: 'פתח במפה', share: 'שתף'
  },
  errors: {
    gpsDenied: 'הרשאת מיקום נדחתה', gpsTimeout: 'פג זמן המתנה ל-GPS',
    noResults: 'לא נמצאו תוצאות', offline: 'אתה לא מחובר',
    bundleMissing: 'חבילת הליבה חסרה'
  },
  warnings: {
    lowConfidence: 'ביטחון נמוך — המערכת לא בטוחה',
    actionBlocked: 'הפעולה נחסמה ע״י מערכת הבטיחות',
    accuracyExceeds: 'דיוק ה-GPS חורג מהגבול השימושי',
    signalLow: 'איכות האות מתחת לסף',
    satellitesFew: 'מעט מדי לוויינים (נדרשים 4)'
  }
};

const AR = {
  _meta: { name: 'العربية', dir: 'rtl', code: 'ar' },
  app: {
    name: 'G.A.N.E ملاح',
    tagline: 'محرك ملاحة ذاتي عالمي',
    searchPlaceholder: 'إلى أين؟ (عنوان، إحداثيات، صوت)'
  },
  mode: {
    FULL: 'تحديد موقع كامل', DEGRADED: 'وضع مقلص',
    DR_ONLY: 'ملاحة بالحساب', LOST: 'فُقد الموقع', RECOVERY: 'يتعافى'
  },
  env: {
    OPEN: 'سماء مفتوحة', URBAN: 'حضري', CANYON: 'وادي حضري',
    TUNNEL: 'نفق', INDOOR: 'داخلي', UNKNOWN: 'غير معروف'
  },
  phase: {
    HEALTHY: 'النظام سليم', CAUTIOUS: 'شذوذ طفيف',
    DEGRADED: 'يعمل بشكل مقلص', RESTRICTED: 'وضع مقيد', FAIL_SAFE: 'وضع الأمان نشط'
  },
  panels: {
    layers: 'طبقات الخريطة', gnss: 'إشارات GNSS', ai: 'مساعد الذكاء الاصطناعي',
    diagnostics: 'التشخيص', replay: 'سجل المهمة'
  },
  layers: {
    street: 'شوارع', dark: 'داكن', satellite: 'قمر صناعي', terrain: 'تضاريس',
    weather: 'رادار الطقس', transit: 'مواصلات', hiking: 'مسارات المشي',
    cacheRegion: 'احفظ المنطقة محلياً'
  },
  diagnostics: {
    kpi: 'المؤشرات', availability: 'التوفر', continuity: 'الاستمرارية',
    modeCycles: 'دورات الوضع', integrityEvents: 'أحداث السلامة',
    engineState: 'حالة المحرك', currentMode: 'الوضع الحالي', ekfCycles: 'دورات EKF',
    uncertainty: 'عدم اليقين', transitions: 'التحولات',
    watchdog: 'الحارس', providers: 'المزودون (قاطع الدائرة)',
    runFull: 'شغّل تشخيص كامل'
  },
  emergency: {
    crashDetected: 'تم اكتشاف حادث',
    severityMinor: 'ارتطام خفيف', severityMajor: 'ارتطام شديد', severitySevere: 'ارتطام بالغ',
    beaconActive: 'منارة الطوارئ نشطة',
    sosButton: 'طوارئ'
  },
  reality: {
    selfCorrecting: 'تصحيح ذاتي', cycle: 'دورة',
    predictions: 'التنبؤات', contradictions: 'التناقضات',
    recurringFailures: 'إخفاقات متكررة', overridesFired: 'تجاوزات نشطة',
    etaCalibration: 'معايرة الوقت'
  },
  consciousness: {
    governing: 'يدير', phase: 'المرحلة', readiness: 'الجاهزية',
    selfConfidence: 'الثقة الذاتية', verdict: 'الحكم', failsafe: 'الأمان-الفشل',
    decisionsBlocked: 'قرارات محجوبة', auditEntries: 'سجلات التدقيق',
    trusting: 'واثق', watchful: 'متيقظ', uncertain: 'غير متأكد', distrustful: 'غير واثق'
  },
  actions: {
    exportMission: 'تصدير JSON', replayMission: 'إعادة تشغيل', clearMission: 'مسح',
    centerOnMe: 'تمركز علي', openInMap: 'افتح في الخريطة', share: 'شارك'
  },
  errors: {
    gpsDenied: 'رُفض إذن الموقع', gpsTimeout: 'انتهت مهلة GPS',
    noResults: 'لا نتائج', offline: 'أنت غير متصل',
    bundleMissing: 'حزمة النواة مفقودة'
  },
  warnings: {
    lowConfidence: 'ثقة منخفضة — النظام غير متأكد',
    actionBlocked: 'تم حجب الإجراء بواسطة نظام السلامة',
    accuracyExceeds: 'دقة GPS تتجاوز الحد المفيد',
    signalLow: 'جودة الإشارة تحت العتبة',
    satellitesFew: 'أقمار صناعية قليلة جداً (مطلوب 4)'
  }
};

const LANGS = { en: EN, he: HE, ar: AR };

function buildTranslator(dict) {
  return function t(key, fallback) {
    const parts = key.split('.');
    let v = dict;
    for (const p of parts) v = v && v[p];
    return v != null ? v : (fallback || key);
  };
}

const I18N = {
  available: Object.keys(LANGS),
  get(code) {
    const lang = LANGS[code] || LANGS[code?.split('-')[0]] || EN;
    return { lang, t: buildTranslator(lang), dir: lang._meta.dir, code: lang._meta.code };
  },
  detect() {
    const nav = typeof navigator !== 'undefined' ? navigator : {};
    const candidates = [nav.language, ...(nav.languages || [])];
    for (const c of candidates) {
      const code = (c || '').toLowerCase().split('-')[0];
      if (LANGS[code]) return code;
    }
    return 'en';
  },
  applyToDOM(code) {
    const { lang, dir } = this.get(code);
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('lang', code);
      document.documentElement.setAttribute('dir', dir);
    }
    return lang;
  }
};

global.GANE_I18N = I18N;
})(typeof window !== 'undefined' ? window : globalThis);
