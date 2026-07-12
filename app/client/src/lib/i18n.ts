/**
 * G.A.N.E — Internationalization System
 * Supports 40+ world languages with RTL support
 * 
 * Translation keys cover ALL navigation UI components:
 * - Navigation modes (drive, walk, emergency, plan)
 * - Sidebar groups & items (NAV, INTEL, OPS, AI, SYS)
 * - Search panel, Route planner, Settings, HUD
 * - Smart panels, Advanced panels, AI copilot
 * - Accessibility, Notifications, Error states
 */

export interface Language {
  code: string;
  name: string;
  nativeName: string;
  dir: 'ltr' | 'rtl';
  flag: string;
}

export const languages: Language[] = [
  { code: 'en', name: 'English', nativeName: 'English', dir: 'ltr', flag: '🇺🇸' },
  { code: 'he', name: 'Hebrew', nativeName: 'עברית', dir: 'rtl', flag: '🇮🇱' },
  { code: 'ar', name: 'Arabic', nativeName: 'العربية', dir: 'rtl', flag: '🇸🇦' },
  { code: 'zh', name: 'Chinese', nativeName: '中文', dir: 'ltr', flag: '🇨🇳' },
  { code: 'ja', name: 'Japanese', nativeName: '日本語', dir: 'ltr', flag: '🇯🇵' },
  { code: 'ko', name: 'Korean', nativeName: '한국어', dir: 'ltr', flag: '🇰🇷' },
  { code: 'hi', name: 'Hindi', nativeName: 'हिन्दी', dir: 'ltr', flag: '🇮🇳' },
  { code: 'es', name: 'Spanish', nativeName: 'Español', dir: 'ltr', flag: '🇪🇸' },
  { code: 'fr', name: 'French', nativeName: 'Français', dir: 'ltr', flag: '🇫🇷' },
  { code: 'de', name: 'German', nativeName: 'Deutsch', dir: 'ltr', flag: '🇩🇪' },
  { code: 'pt', name: 'Portuguese', nativeName: 'Português', dir: 'ltr', flag: '🇧🇷' },
  { code: 'ru', name: 'Russian', nativeName: 'Русский', dir: 'ltr', flag: '🇷🇺' },
  { code: 'it', name: 'Italian', nativeName: 'Italiano', dir: 'ltr', flag: '🇮🇹' },
  { code: 'nl', name: 'Dutch', nativeName: 'Nederlands', dir: 'ltr', flag: '🇳🇱' },
  { code: 'pl', name: 'Polish', nativeName: 'Polski', dir: 'ltr', flag: '🇵🇱' },
  { code: 'tr', name: 'Turkish', nativeName: 'Türkçe', dir: 'ltr', flag: '🇹🇷' },
  { code: 'sv', name: 'Swedish', nativeName: 'Svenska', dir: 'ltr', flag: '🇸🇪' },
  { code: 'da', name: 'Danish', nativeName: 'Dansk', dir: 'ltr', flag: '🇩🇰' },
  { code: 'fi', name: 'Finnish', nativeName: 'Suomi', dir: 'ltr', flag: '🇫🇮' },
  { code: 'no', name: 'Norwegian', nativeName: 'Norsk', dir: 'ltr', flag: '🇳🇴' },
  { code: 'uk', name: 'Ukrainian', nativeName: 'Українська', dir: 'ltr', flag: '🇺🇦' },
  { code: 'cs', name: 'Czech', nativeName: 'Čeština', dir: 'ltr', flag: '🇨🇿' },
  { code: 'ro', name: 'Romanian', nativeName: 'Română', dir: 'ltr', flag: '🇷🇴' },
  { code: 'el', name: 'Greek', nativeName: 'Ελληνικά', dir: 'ltr', flag: '🇬🇷' },
  { code: 'th', name: 'Thai', nativeName: 'ไทย', dir: 'ltr', flag: '🇹🇭' },
  { code: 'vi', name: 'Vietnamese', nativeName: 'Tiếng Việt', dir: 'ltr', flag: '🇻🇳' },
  { code: 'id', name: 'Indonesian', nativeName: 'Bahasa Indonesia', dir: 'ltr', flag: '🇮🇩' },
  { code: 'ms', name: 'Malay', nativeName: 'Bahasa Melayu', dir: 'ltr', flag: '🇲🇾' },
  { code: 'fa', name: 'Persian', nativeName: 'فارسی', dir: 'rtl', flag: '🇮🇷' },
  { code: 'ur', name: 'Urdu', nativeName: 'اردو', dir: 'rtl', flag: '🇵🇰' },
  { code: 'bn', name: 'Bengali', nativeName: 'বাংলা', dir: 'ltr', flag: '🇧🇩' },
  { code: 'sw', name: 'Swahili', nativeName: 'Kiswahili', dir: 'ltr', flag: '🇰🇪' },
  { code: 'hu', name: 'Hungarian', nativeName: 'Magyar', dir: 'ltr', flag: '🇭🇺' },
  { code: 'bg', name: 'Bulgarian', nativeName: 'Български', dir: 'ltr', flag: '🇧🇬' },
  { code: 'hr', name: 'Croatian', nativeName: 'Hrvatski', dir: 'ltr', flag: '🇭🇷' },
  { code: 'sk', name: 'Slovak', nativeName: 'Slovenčina', dir: 'ltr', flag: '🇸🇰' },
  { code: 'lt', name: 'Lithuanian', nativeName: 'Lietuvių', dir: 'ltr', flag: '🇱🇹' },
  { code: 'lv', name: 'Latvian', nativeName: 'Latviešu', dir: 'ltr', flag: '🇱🇻' },
  { code: 'et', name: 'Estonian', nativeName: 'Eesti', dir: 'ltr', flag: '🇪🇪' },
  { code: 'sl', name: 'Slovenian', nativeName: 'Slovenščina', dir: 'ltr', flag: '🇸🇮' },
  { code: 'ca', name: 'Catalan', nativeName: 'Català', dir: 'ltr', flag: '🏴' },
  { code: 'tl', name: 'Filipino', nativeName: 'Filipino', dir: 'ltr', flag: '🇵🇭' },
  { code: 'af', name: 'Afrikaans', nativeName: 'Afrikaans', dir: 'ltr', flag: '🇿🇦' },
];

// ═══════════════════════════════════════════════════════════
// TRANSLATION KEY TYPE — All UI strings in the application
// ═══════════════════════════════════════════════════════════
export type TranslationKey =
  // Navigation & App
  | 'nav.home' | 'nav.spec' | 'nav.gane' | 'nav.search'
  | 'home.title' | 'home.subtitle' | 'home.cta' | 'home.explore'
  | 'spec.title' | 'spec.search' | 'spec.sections' | 'spec.parts'
  | 'spec.previous' | 'spec.next' | 'spec.toc'
  | 'gane.title' | 'gane.subtitle' | 'gane.features' | 'gane.layer'
  | 'gane.buildOrder' | 'gane.systemDef'
  // Voice
  | 'voice.play' | 'voice.pause' | 'voice.stop' | 'voice.speed' | 'voice.language'
  // Language
  | 'lang.select' | 'lang.current'
  // Common
  | 'common.loading' | 'common.error' | 'common.close' | 'common.back'
  | 'common.readAloud' | 'common.download' | 'common.share'
  | 'common.save' | 'common.cancel' | 'common.confirm' | 'common.delete'
  | 'common.search' | 'common.settings' | 'common.navigate'
  | 'common.on' | 'common.off' | 'common.yes' | 'common.no'
  // Stats
  | 'stats.parts' | 'stats.sections' | 'stats.features' | 'stats.languages'
  | 'stats.entities' | 'stats.events' | 'stats.phases'
  // Navigation Modes
  | 'mode.drive' | 'mode.walk' | 'mode.emergency' | 'mode.plan'
  // Sidebar Groups
  | 'group.nav' | 'group.intel' | 'group.ops' | 'group.ai' | 'group.sys'
  // Sidebar Items — NAV
  | 'sidebar.layers' | 'sidebar.parking' | 'sidebar.charging' | 'sidebar.transport'
  | 'sidebar.indoor' | 'sidebar.arNav' | 'sidebar.history' | 'sidebar.draw'
  // Sidebar Items — INTEL
  | 'sidebar.weather' | 'sidebar.satellite' | 'sidebar.imagery' | 'sidebar.gnss'
  | 'sidebar.v2x' | 'sidebar.twin' | 'sidebar.risk' | 'sidebar.coverage'
  // Sidebar Items — OPS
  | 'sidebar.cmd' | 'sidebar.eoc' | 'sidebar.fleet' | 'sidebar.cameras'
  | 'sidebar.report' | 'sidebar.incident' | 'sidebar.alerts' | 'sidebar.c4isr' | 'sidebar.radio'
  // Sidebar Items — AI
  | 'sidebar.optimizer' | 'sidebar.score' | 'sidebar.analytics' | 'sidebar.social'
  | 'sidebar.liveShare' | 'sidebar.collab' | 'sidebar.liveTeam'
  // Sidebar Items — SYS
  | 'sidebar.arch' | 'sidebar.pipeline' | 'sidebar.spec' | 'sidebar.wallet'
  | 'sidebar.offline' | 'sidebar.offlineMaps' | 'sidebar.a11y' | 'sidebar.engine' | 'sidebar.battery'
  // Bottom Dock
  | 'dock.traffic' | 'dock.settings' | 'dock.admin' | 'dock.more'
  // Search Panel
  | 'search.whereTo' | 'search.listening' | 'search.results' | 'search.nearby'
  | 'search.favorites' | 'search.recent' | 'search.suggestions'
  | 'search.food' | 'search.gas' | 'search.coffee' | 'search.parking'
  | 'search.shopping' | 'search.hospital'
  | 'search.bestTime' | 'search.parkingAvailable' | 'search.goodWeather'
  // Route Planner
  | 'route.drive' | 'route.walk' | 'route.emergency' | 'route.plan'
  | 'route.start' | 'route.destination' | 'route.addStop'
  | 'route.fastest' | 'route.shortest' | 'route.scenic'
  // Settings
  | 'settings.voiceGuidance' | 'settings.alertSounds' | 'settings.hudDisplay'
  | 'settings.speedDisplay' | 'settings.animations' | 'settings.offlineMaps'
  | 'settings.locationSharing' | 'settings.language' | 'settings.theme'
  | 'settings.notifications' | 'settings.accessibility'
  // Notification types
  | 'notif.info' | 'notif.success' | 'notif.warning' | 'notif.error'
  | 'notif.system' | 'notif.collab' | 'notif.admin'
  | 'notif.sound' | 'notif.toast'
  // Battery & Power
  | 'power.performance' | 'power.balanced' | 'power.saver' | 'power.ultraSaver'
  // Accessibility
  | 'a11y.title' | 'a11y.highContrast' | 'a11y.reduceMotion'
  | 'a11y.fontSize' | 'a11y.colorBlind' | 'a11y.screenReader'
  | 'a11y.skipToContent'
  | 'a11y.normal' | 'a11y.large' | 'a11y.extraLarge'
  // AI Copilot
  | 'ai.greeting' | 'ai.subtitle' | 'ai.placeholder' | 'ai.noInsights'
  | 'ai.chat' | 'ai.insights'
  // Incident Reporter
  | 'incident.accident' | 'incident.roadblock' | 'incident.police'
  | 'incident.hazard' | 'incident.construction' | 'incident.flooding'
  | 'incident.speedTrap' | 'incident.roadClosed'
  | 'incident.liveFeed' | 'incident.report' | 'incident.stats'
  // Map Layers
  | 'layer.traffic' | 'layer.transit' | 'layer.cycling' | 'layer.satellite'
  | 'layer.terrain' | 'layer.weatherRadar' | 'layer.temperature'
  | 'layer.wind' | 'layer.precipitation' | 'layer.airQuality'
  | 'layer.earthquake' | 'layer.riskZones' | 'layer.nightVision'
  | 'layer.heatmap' | 'layer.evCharging' | 'layer.smartParking'
  | 'layer.buildings3d' | 'layer.indoorMaps' | 'layer.laneGuidance'
  // Map Themes
  | 'theme.ganeDark' | 'theme.midnight' | 'theme.satellite'
  | 'theme.topographic' | 'theme.nightVision' | 'theme.thermal'
  // Layer Categories
  | 'layerCat.core' | 'layerCat.weather' | 'layerCat.safety'
  | 'layerCat.infrastructure' | 'layerCat.advanced'
  // Multi-modal transport
  | 'transport.drive' | 'transport.transit' | 'transport.bike'
  | 'transport.walk' | 'transport.train' | 'transport.scooter'
  // Driver Score
  | 'score.safety' | 'score.alertness' | 'score.smoothness' | 'score.eco'
  // Evidence/Report types
  | 'evidence.accident' | 'evidence.hazard' | 'evidence.police'
  | 'evidence.closure' | 'evidence.pothole' | 'evidence.weather'
  // Live Sharing
  | 'share.viewOnly' | 'share.viewEta' | 'share.fullAccess'
  // EOC Severity
  | 'severity.critical' | 'severity.high' | 'severity.medium' | 'severity.low'
  // EOC Alert Levels
  | 'alert.green' | 'alert.yellow' | 'alert.orange' | 'alert.red'
  // Boot Sequence
  | 'boot.quantumCore' | 'boot.systemCrates' | 'boot.neuralRouting'
  | 'boot.gnssLock' | 'boot.dataStreams' | 'boot.holoUI'
  // Data Pipeline
  | 'pipeline.weather' | 'pipeline.airQuality' | 'pipeline.seismic'
  | 'pipeline.position' | 'pipeline.elevation' | 'pipeline.maps'
  // C4ISR
  | 'c4isr.fleet' | 'c4isr.missions' | 'c4isr.vrp'
  // Analytics tabs
  | 'analytics.overview' | 'analytics.weather' | 'analytics.seismic' | 'analytics.network'
  | 'analytics.title' | 'analytics.subtitle' | 'analytics.systemHealth'
  // Error Boundary
  | 'error.componentFailed' | 'error.tryAgain' | 'error.goHome'
  // Panels
  | 'panel.gnssManager' | 'panel.indoorPos' | 'panel.arNav' | 'panel.riskEngine'
  // Risk factors
  | 'risk.weather' | 'risk.seismic' | 'risk.position'
  // Metrics
  | 'metric.temperature' | 'metric.humidity' | 'metric.windSpeed' | 'metric.pressure';

type Translations = Record<TranslationKey, string>;

// ═══════════════════════════════════════════════════════════
// ENGLISH (Base Language)
// ═══════════════════════════════════════════════════════════
const en: Translations = {
  // Navigation & App
  'nav.home': 'Home', 'nav.spec': 'G.A.N.E Specification', 'nav.gane': 'G.A.N.E', 'nav.search': 'Search specification...',
  'home.title': 'Global Advanced Navigation Engine', 'home.subtitle': 'Comprehensive Engineering Master Specification for a decentralized, defense-grade infrastructure managing global traffic networks at continental scale.',
  'home.cta': 'Explore Specification', 'home.explore': 'Explore G.A.N.E',
  'spec.title': 'Engineering Specification', 'spec.search': 'Search specification...', 'spec.sections': 'sections', 'spec.parts': 'parts',
  'spec.previous': 'Previous', 'spec.next': 'Next', 'spec.toc': 'Table of Contents',
  'gane.title': 'G.A.N.E Navigator', 'gane.subtitle': 'Global Advanced Navigation Engine — Complete System Architecture',
  'gane.features': 'Features', 'gane.layer': 'System Layer', 'gane.buildOrder': 'Build Order', 'gane.systemDef': 'System Definition',
  // Voice
  'voice.play': 'Read Aloud', 'voice.pause': 'Pause', 'voice.stop': 'Stop', 'voice.speed': 'Speed', 'voice.language': 'Voice Language',
  // Language
  'lang.select': 'Select Language', 'lang.current': 'Current Language',
  // Common
  'common.loading': 'Loading...', 'common.error': 'Error', 'common.close': 'Close', 'common.back': 'Back',
  'common.readAloud': 'Read Aloud', 'common.download': 'Download', 'common.share': 'Share',
  'common.save': 'Save', 'common.cancel': 'Cancel', 'common.confirm': 'Confirm', 'common.delete': 'Delete',
  'common.search': 'Search', 'common.settings': 'Settings', 'common.navigate': 'Navigate',
  'common.on': 'On', 'common.off': 'Off', 'common.yes': 'Yes', 'common.no': 'No',
  // Stats
  'stats.parts': 'Specification Parts', 'stats.sections': 'Technical Sections', 'stats.features': 'System Features',
  'stats.languages': 'Languages', 'stats.entities': 'Core Entities', 'stats.events': 'Event Types', 'stats.phases': 'Build Phases',
  // Navigation Modes
  'mode.drive': 'Drive', 'mode.walk': 'Walk', 'mode.emergency': 'SOS', 'mode.plan': 'Plan',
  // Sidebar Groups
  'group.nav': 'NAV', 'group.intel': 'INTEL', 'group.ops': 'OPS', 'group.ai': 'AI', 'group.sys': 'SYS',
  // Sidebar Items — NAV
  'sidebar.layers': 'Layers', 'sidebar.parking': 'Parking', 'sidebar.charging': 'Charging', 'sidebar.transport': 'Transport',
  'sidebar.indoor': 'Indoor', 'sidebar.arNav': 'AR Nav', 'sidebar.history': 'History', 'sidebar.draw': 'Draw',
  // Sidebar Items — INTEL
  'sidebar.weather': 'Weather', 'sidebar.satellite': 'Satellite', 'sidebar.imagery': 'Imagery', 'sidebar.gnss': 'GNSS',
  'sidebar.v2x': 'V2X', 'sidebar.twin': 'Twin', 'sidebar.risk': 'Risk', 'sidebar.coverage': 'Coverage',
  // Sidebar Items — OPS
  'sidebar.cmd': 'CMD', 'sidebar.eoc': 'EOC', 'sidebar.fleet': 'Fleet', 'sidebar.cameras': 'Cameras',
  'sidebar.report': 'Report', 'sidebar.incident': 'Incident', 'sidebar.alerts': 'Alerts', 'sidebar.c4isr': 'C4ISR', 'sidebar.radio': 'Radio',
  // Sidebar Items — AI
  'sidebar.optimizer': 'Optimizer', 'sidebar.score': 'Score', 'sidebar.analytics': 'Analytics', 'sidebar.social': 'Social',
  'sidebar.liveShare': 'Share', 'sidebar.collab': 'Collab', 'sidebar.liveTeam': 'Live Team',
  // Sidebar Items — SYS
  'sidebar.arch': 'Arch', 'sidebar.pipeline': 'Pipeline', 'sidebar.spec': 'Spec', 'sidebar.wallet': 'Wallet',
  'sidebar.offline': 'Offline', 'sidebar.offlineMaps': 'Offline Maps', 'sidebar.a11y': 'A11y', 'sidebar.engine': 'G.A.N.E', 'sidebar.battery': 'Battery',
  // Bottom Dock
  'dock.traffic': 'Traffic', 'dock.settings': 'Settings', 'dock.admin': 'Admin', 'dock.more': 'More',
  // Search Panel
  'search.whereTo': 'Where to?', 'search.listening': 'Listening...', 'search.results': 'Results',
  'search.nearby': 'Nearby Places', 'search.favorites': 'Favorites', 'search.recent': 'Recent Searches',
  'search.suggestions': 'Smart Suggestions',
  'search.food': 'Food', 'search.gas': 'Gas', 'search.coffee': 'Coffee', 'search.parking': 'Parking',
  'search.shopping': 'Shopping', 'search.hospital': 'Hospital',
  'search.bestTime': 'Best time to drive to work: now', 'search.parkingAvailable': 'Parking available near your last destination',
  'search.goodWeather': 'Good weather for a walk',
  // Route Planner
  'route.drive': 'Drive', 'route.walk': 'Walk', 'route.emergency': 'SOS', 'route.plan': 'Plan',
  'route.start': 'Start', 'route.destination': 'Destination', 'route.addStop': 'Add Stop',
  'route.fastest': 'Fastest', 'route.shortest': 'Shortest', 'route.scenic': 'Scenic',
  // Settings
  'settings.voiceGuidance': 'Voice Guidance', 'settings.alertSounds': 'Alert Sounds',
  'settings.hudDisplay': 'HUD Display', 'settings.speedDisplay': 'Speed Display',
  'settings.animations': 'Animations', 'settings.offlineMaps': 'Offline Maps',
  'settings.locationSharing': 'Location Sharing', 'settings.language': 'Language',
  'settings.theme': 'Theme', 'settings.notifications': 'Notifications', 'settings.accessibility': 'Accessibility',
  // Notification types
  'notif.info': 'Info', 'notif.success': 'Success', 'notif.warning': 'Warning', 'notif.error': 'Error',
  'notif.system': 'System', 'notif.collab': 'Collaboration', 'notif.admin': 'Admin',
  'notif.sound': 'Sound', 'notif.toast': 'Toast Pop-ups',
  // Battery & Power
  'power.performance': 'Performance', 'power.balanced': 'Balanced', 'power.saver': 'Power Saver', 'power.ultraSaver': 'Ultra Saver',
  // Accessibility
  'a11y.title': 'Accessibility', 'a11y.highContrast': 'High Contrast', 'a11y.reduceMotion': 'Reduce Motion',
  'a11y.fontSize': 'Font Size', 'a11y.colorBlind': 'Color Blind Mode', 'a11y.screenReader': 'Screen Reader',
  'a11y.skipToContent': 'Skip to main content',
  'a11y.normal': 'Normal', 'a11y.large': 'Large', 'a11y.extraLarge': 'Extra Large',
  // AI Copilot
  'ai.greeting': 'Hello, I am G.A.N.E AI', 'ai.subtitle': 'Ask me about routes, traffic, weather, parking...',
  'ai.placeholder': 'Ask G.A.N.E AI...', 'ai.noInsights': 'No insights right now',
  'ai.chat': 'Chat', 'ai.insights': 'Insights',
  // Incident Reporter
  'incident.accident': 'Accident', 'incident.roadblock': 'Roadblock', 'incident.police': 'Police',
  'incident.hazard': 'Hazard', 'incident.construction': 'Construction', 'incident.flooding': 'Flooding',
  'incident.speedTrap': 'Speed Trap', 'incident.roadClosed': 'Road Closed',
  'incident.liveFeed': 'Live Feed', 'incident.report': 'Report', 'incident.stats': 'Stats',
  // Map Layers
  'layer.traffic': 'Live Traffic', 'layer.transit': 'Public Transit', 'layer.cycling': 'Cycling Network',
  'layer.satellite': 'Satellite Imagery', 'layer.terrain': 'Terrain & Elevation',
  'layer.weatherRadar': 'Precipitation Radar', 'layer.temperature': 'Temperature Map',
  'layer.wind': 'Wind Patterns', 'layer.precipitation': 'Precipitation Forecast', 'layer.airQuality': 'Air Quality Index',
  'layer.earthquake': 'Seismic Activity', 'layer.riskZones': 'Risk Zones', 'layer.nightVision': 'Night Vision',
  'layer.heatmap': 'Activity Heatmap', 'layer.evCharging': 'EV Charging', 'layer.smartParking': 'Smart Parking',
  'layer.buildings3d': '3D Buildings', 'layer.indoorMaps': 'Indoor Maps', 'layer.laneGuidance': 'Lane Guidance',
  // Map Themes
  'theme.ganeDark': 'G.A.N.E Dark', 'theme.midnight': 'Midnight', 'theme.satellite': 'Satellite',
  'theme.topographic': 'Topographic', 'theme.nightVision': 'Night Vision', 'theme.thermal': 'Thermal',
  // Layer Categories
  'layerCat.core': 'Core Navigation', 'layerCat.weather': 'Weather & Atmosphere',
  'layerCat.safety': 'Safety & Risk', 'layerCat.infrastructure': 'Infrastructure', 'layerCat.advanced': 'Advanced / Experimental',
  // Multi-modal transport
  'transport.drive': 'Drive', 'transport.transit': 'Transit', 'transport.bike': 'Bike',
  'transport.walk': 'Walk', 'transport.train': 'Train', 'transport.scooter': 'Scooter',
  // Driver Score
  'score.safety': 'Safety', 'score.alertness': 'Alertness', 'score.smoothness': 'Smoothness', 'score.eco': 'Eco Driving',
  // Evidence/Report types
  'evidence.accident': 'Accident', 'evidence.hazard': 'Road Hazard', 'evidence.police': 'Police',
  'evidence.closure': 'Road Closed', 'evidence.pothole': 'Pothole', 'evidence.weather': 'Weather',
  // Live Sharing
  'share.viewOnly': 'View Only', 'share.viewEta': 'View + ETA', 'share.fullAccess': 'Full Access',
  // EOC Severity
  'severity.critical': 'CRITICAL', 'severity.high': 'HIGH', 'severity.medium': 'MEDIUM', 'severity.low': 'LOW',
  // EOC Alert Levels
  'alert.green': 'GREEN — Normal', 'alert.yellow': 'YELLOW — Elevated', 'alert.orange': 'ORANGE — Emergency', 'alert.red': 'RED — Critical',
  // Boot Sequence
  'boot.quantumCore': 'Quantum Core', 'boot.systemCrates': 'System Crates', 'boot.neuralRouting': 'Neural Routing Engine',
  'boot.gnssLock': 'GNSS Constellation Lock', 'boot.dataStreams': 'Real-Time Data Streams', 'boot.holoUI': 'Holographic UI Layer',
  // Data Pipeline
  'pipeline.weather': 'Weather', 'pipeline.airQuality': 'Air Quality', 'pipeline.seismic': 'Seismic',
  'pipeline.position': 'Position', 'pipeline.elevation': 'Elevation', 'pipeline.maps': 'Maps',
  // C4ISR
  'c4isr.fleet': 'FLEET', 'c4isr.missions': 'MISSIONS', 'c4isr.vrp': 'VRP',
  // Analytics tabs
  'analytics.overview': 'Overview', 'analytics.weather': 'Weather', 'analytics.seismic': 'Seismic', 'analytics.network': 'Network',
  'analytics.title': 'Analytics Engine', 'analytics.subtitle': 'Real-time data analysis', 'analytics.systemHealth': 'System Health',
  // Error Boundary
  'error.componentFailed': 'Component failed to load', 'error.tryAgain': 'Try Again', 'error.goHome': 'Go Home',
  // Panels
  'panel.gnssManager': 'GNSS Manager', 'panel.indoorPos': 'Indoor Positioning', 'panel.arNav': 'AR Navigation', 'panel.riskEngine': 'Risk Engine',
  // Risk factors
  'risk.weather': 'Weather Risk', 'risk.seismic': 'Seismic Risk', 'risk.position': 'Position Risk',
  // Metrics
  'metric.temperature': 'Temperature', 'metric.humidity': 'Humidity', 'metric.windSpeed': 'Wind Speed', 'metric.pressure': 'Pressure',
};

// ═══════════════════════════════════════════════════════════
// HEBREW
// ═══════════════════════════════════════════════════════════
const he: Translations = {
  'nav.home': 'בית', 'nav.spec': 'מפרט G.A.N.E', 'nav.gane': 'G.A.N.E', 'nav.search': 'חיפוש במפרט...',
  'home.title': 'מנוע ניווט מתקדם גלובלי', 'home.subtitle': 'מפרט הנדסי מלא למערכת תשתית מבוזרת ברמת הגנה לניהול רשתות תנועה גלובליות בקנה מידה יבשתי.',
  'home.cta': 'צפייה במפרט', 'home.explore': 'חקור G.A.N.E',
  'spec.title': 'מפרט הנדסי', 'spec.search': 'חיפוש במפרט...', 'spec.sections': 'סעיפים', 'spec.parts': 'חלקים',
  'spec.previous': 'הקודם', 'spec.next': 'הבא', 'spec.toc': 'תוכן עניינים',
  'gane.title': 'G.A.N.E Navigator', 'gane.subtitle': 'מנוע ניווט מתקדם גלובלי — ארכיטקטורת מערכת מלאה',
  'gane.features': 'יכולות', 'gane.layer': 'שכבת מערכת', 'gane.buildOrder': 'סדר בנייה', 'gane.systemDef': 'הגדרת המערכת',
  'voice.play': 'קריאה בקול', 'voice.pause': 'השהייה', 'voice.stop': 'עצירה', 'voice.speed': 'מהירות', 'voice.language': 'שפת קריאה',
  'lang.select': 'בחירת שפה', 'lang.current': 'שפה נוכחית',
  'common.loading': 'טוען...', 'common.error': 'שגיאה', 'common.close': 'סגירה', 'common.back': 'חזרה',
  'common.readAloud': 'קריאה בקול', 'common.download': 'הורדה', 'common.share': 'שיתוף',
  'common.save': 'שמירה', 'common.cancel': 'ביטול', 'common.confirm': 'אישור', 'common.delete': 'מחיקה',
  'common.search': 'חיפוש', 'common.settings': 'הגדרות', 'common.navigate': 'נווט',
  'common.on': 'פעיל', 'common.off': 'כבוי', 'common.yes': 'כן', 'common.no': 'לא',
  'stats.parts': 'חלקי מפרט', 'stats.sections': 'סעיפים טכניים', 'stats.features': 'יכולות מערכת',
  'stats.languages': 'שפות', 'stats.entities': 'ישויות ליבה', 'stats.events': 'סוגי אירועים', 'stats.phases': 'שלבי בנייה',
  // Navigation Modes
  'mode.drive': 'נהיגה', 'mode.walk': 'הליכה', 'mode.emergency': 'חירום', 'mode.plan': 'תכנון',
  // Sidebar Groups
  'group.nav': 'ניווט', 'group.intel': 'מודיעין', 'group.ops': 'מבצעים', 'group.ai': 'בינה', 'group.sys': 'מערכת',
  // Sidebar Items — NAV
  'sidebar.layers': 'שכבות', 'sidebar.parking': 'חניה', 'sidebar.charging': 'טעינה', 'sidebar.transport': 'תחבורה',
  'sidebar.indoor': 'פנימי', 'sidebar.arNav': 'AR', 'sidebar.history': 'היסטוריה', 'sidebar.draw': 'ציור',
  // Sidebar Items — INTEL
  'sidebar.weather': 'מזג אוויר', 'sidebar.satellite': 'לוויין', 'sidebar.imagery': 'תצלומי לוויין', 'sidebar.gnss': 'GNSS',
  'sidebar.v2x': 'V2X', 'sidebar.twin': 'תאום', 'sidebar.risk': 'סיכון', 'sidebar.coverage': 'כיסוי',
  // Sidebar Items — OPS
  'sidebar.cmd': 'פיקוד', 'sidebar.eoc': 'חירום', 'sidebar.fleet': 'צי', 'sidebar.cameras': 'מצלמות',
  'sidebar.report': 'דיווח', 'sidebar.incident': 'אירוע', 'sidebar.alerts': 'התראות', 'sidebar.c4isr': 'פיקוד', 'sidebar.radio': 'רדיו',
  // Sidebar Items — AI
  'sidebar.optimizer': 'אופטימיזציה', 'sidebar.score': 'ציון', 'sidebar.analytics': 'אנליטיקה', 'sidebar.social': 'חברתי',
  'sidebar.liveShare': 'שיתוף', 'sidebar.collab': 'שיתוף פעולה', 'sidebar.liveTeam': 'צוות חי',
  // Sidebar Items — SYS
  'sidebar.arch': 'ארכיטקטורה', 'sidebar.pipeline': 'צנרת', 'sidebar.spec': 'מפרט', 'sidebar.wallet': 'ארנק',
  'sidebar.offline': 'אופליין', 'sidebar.offlineMaps': 'מפות אופליין', 'sidebar.a11y': 'נגישות', 'sidebar.engine': 'מנוע', 'sidebar.battery': 'סוללה',
  // Bottom Dock
  'dock.traffic': 'תנועה', 'dock.settings': 'הגדרות', 'dock.admin': 'אדמין', 'dock.more': 'עוד',
  // Search Panel
  'search.whereTo': 'לאן נוסעים?', 'search.listening': 'מקשיב...', 'search.results': 'תוצאות',
  'search.nearby': 'מקומות קרובים', 'search.favorites': 'מועדפים', 'search.recent': 'חיפושים אחרונים',
  'search.suggestions': 'הצעות חכמות',
  'search.food': 'מסעדות', 'search.gas': 'דלק', 'search.coffee': 'קפה', 'search.parking': 'חניה',
  'search.shopping': 'קניות', 'search.hospital': 'רפואה',
  'search.bestTime': 'הזמן הטוב ביותר לנסוע לעבודה: עכשיו', 'search.parkingAvailable': 'חניה פנויה ליד היעד האחרון שלך',
  'search.goodWeather': 'מזג אוויר נוח לטיול ברגל',
  // Route Planner
  'route.drive': 'נהיגה', 'route.walk': 'הליכה', 'route.emergency': 'חירום', 'route.plan': 'תכנון',
  'route.start': 'התחלה', 'route.destination': 'יעד', 'route.addStop': 'הוסף עצירה',
  'route.fastest': 'מהיר', 'route.shortest': 'קצר', 'route.scenic': 'נופי',
  // Settings
  'settings.voiceGuidance': 'הנחיות קוליות', 'settings.alertSounds': 'צלילי התראה',
  'settings.hudDisplay': 'תצוגת HUD', 'settings.speedDisplay': 'תצוגת מהירות',
  'settings.animations': 'אנימציות', 'settings.offlineMaps': 'מפות אופליין',
  'settings.locationSharing': 'שיתוף מיקום', 'settings.language': 'שפה',
  'settings.theme': 'ערכת נושא', 'settings.notifications': 'התראות', 'settings.accessibility': 'נגישות',
  // Notification types
  'notif.info': 'מידע', 'notif.success': 'הצלחה', 'notif.warning': 'אזהרה', 'notif.error': 'שגיאה',
  'notif.system': 'מערכת', 'notif.collab': 'שיתוף', 'notif.admin': 'ניהול',
  'notif.sound': 'צלילים', 'notif.toast': 'חלוניות קופצות',
  // Battery & Power
  'power.performance': 'ביצועים', 'power.balanced': 'מאוזן', 'power.saver': 'חיסכון', 'power.ultraSaver': 'חיסכון קיצוני',
  // Accessibility
  'a11y.title': 'נגישות', 'a11y.highContrast': 'ניגודיות גבוהה', 'a11y.reduceMotion': 'הפחתת תנועה',
  'a11y.fontSize': 'גודל גופן', 'a11y.colorBlind': 'מצב עיוורון צבעים', 'a11y.screenReader': 'קורא מסך',
  'a11y.skipToContent': 'דלג לתוכן הראשי',
  'a11y.normal': 'רגיל', 'a11y.large': 'גדול', 'a11y.extraLarge': 'גדול מאוד',
  // AI Copilot
  'ai.greeting': 'שלום, אני G.A.N.E AI', 'ai.subtitle': 'שאל אותי על מסלולים, תנועה, מזג אוויר, חניה...',
  'ai.placeholder': 'שאל את G.A.N.E AI...', 'ai.noInsights': 'אין תובנות כרגע',
  'ai.chat': 'שיחה', 'ai.insights': 'תובנות',
  // Incident Reporter
  'incident.accident': 'תאונה', 'incident.roadblock': 'חסימה', 'incident.police': 'משטרה',
  'incident.hazard': 'מפגע', 'incident.construction': 'עבודות', 'incident.flooding': 'הצפה',
  'incident.speedTrap': 'מצלמת מהירות', 'incident.roadClosed': 'כביש סגור',
  'incident.liveFeed': 'עדכונים', 'incident.report': 'דיווח', 'incident.stats': 'סטטיסטיקה',
  // Map Layers
  'layer.traffic': 'תנועה חיה', 'layer.transit': 'תחבורה ציבורית', 'layer.cycling': 'רשת אופניים',
  'layer.satellite': 'תצלום לוויין', 'layer.terrain': 'טופוגרפיה וגובה',
  'layer.weatherRadar': 'רדאר משקעים', 'layer.temperature': 'מפת טמפרטורה',
  'layer.wind': 'דפוסי רוח', 'layer.precipitation': 'תחזית משקעים', 'layer.airQuality': 'מדד איכות אוויר',
  'layer.earthquake': 'פעילות סייסמית', 'layer.riskZones': 'אזורי סיכון', 'layer.nightVision': 'ראיית לילה',
  'layer.heatmap': 'מפת חום פעילות', 'layer.evCharging': 'עמדות טעינה', 'layer.smartParking': 'חניה חכמה',
  'layer.buildings3d': 'בניינים תלת-ממד', 'layer.indoorMaps': 'מפות פנים', 'layer.laneGuidance': 'הנחיית נתיבים',
  // Map Themes
  'theme.ganeDark': 'G.A.N.E כהה', 'theme.midnight': 'חצות', 'theme.satellite': 'לוויין',
  'theme.topographic': 'טופוגרפי', 'theme.nightVision': 'ראיית לילה', 'theme.thermal': 'תרמי',
  // Layer Categories
  'layerCat.core': 'ניווט ליבה', 'layerCat.weather': 'מזג אוויר ואטמוספרה',
  'layerCat.safety': 'בטיחות וסיכונים', 'layerCat.infrastructure': 'תשתיות', 'layerCat.advanced': 'מתקדם / ניסיוני',
  // Multi-modal transport
  'transport.drive': 'נהיגה', 'transport.transit': 'תח"צ', 'transport.bike': 'אופניים',
  'transport.walk': 'הליכה', 'transport.train': 'רכבת', 'transport.scooter': 'קורקינט',
  // Driver Score
  'score.safety': 'בטיחות', 'score.alertness': 'ערנות', 'score.smoothness': 'חלקות', 'score.eco': 'נהיגה ירוקה',
  // Evidence/Report types
  'evidence.accident': 'תאונה', 'evidence.hazard': 'מפגע', 'evidence.police': 'משטרה',
  'evidence.closure': 'כביש חסום', 'evidence.pothole': 'בור', 'evidence.weather': 'מזג אוויר',
  // Live Sharing
  'share.viewOnly': 'צפייה בלבד', 'share.viewEta': 'צפייה + זמן הגעה', 'share.fullAccess': 'גישה מלאה',
  // EOC Severity
  'severity.critical': 'קריטי', 'severity.high': 'גבוה', 'severity.medium': 'בינוני', 'severity.low': 'נמוך',
  // EOC Alert Levels
  'alert.green': 'ירוק — שגרה', 'alert.yellow': 'צהוב — מוגבר', 'alert.orange': 'כתום — חירום', 'alert.red': 'אדום — קריטי',
  // Boot Sequence
  'boot.quantumCore': 'ליבת קוונטית', 'boot.systemCrates': 'מודולי מערכת', 'boot.neuralRouting': 'מנוע ניתוב עצבי',
  'boot.gnssLock': 'נעילת לוויינים', 'boot.dataStreams': 'זרימות נתונים חיות', 'boot.holoUI': 'שכבת הולוגרפית',
  // Data Pipeline
  'pipeline.weather': 'מזג אוויר', 'pipeline.airQuality': 'איכות אוויר', 'pipeline.seismic': 'סייסמי',
  'pipeline.position': 'מיקום', 'pipeline.elevation': 'גובה', 'pipeline.maps': 'מפות',
  // C4ISR
  'c4isr.fleet': 'צי', 'c4isr.missions': 'משימות', 'c4isr.vrp': 'אופטימיזציה',
  // Analytics tabs
  'analytics.overview': 'סקירה', 'analytics.weather': 'מזג אוויר', 'analytics.seismic': 'סייסמי', 'analytics.network': 'רשת',
  'analytics.title': 'מנוע אנליטיקה', 'analytics.subtitle': 'ניתוח נתונים בזמן אמת', 'analytics.systemHealth': 'בריאות מערכת',
  // Error Boundary
  'error.componentFailed': 'הרכיב נכשל בטעינה', 'error.tryAgain': 'נסה שוב', 'error.goHome': 'חזרה לבית',
  // Panels
  'panel.gnssManager': 'מנהל לוויינים', 'panel.indoorPos': 'מיקום פנימי', 'panel.arNav': 'ניווט AR', 'panel.riskEngine': 'מנוע סיכונים',
  // Risk factors
  'risk.weather': 'סיכון מזג אוויר', 'risk.seismic': 'סיכון סיסמי', 'risk.position': 'סיכון מיקום',
  // Metrics
  'metric.temperature': 'טמפרטורה', 'metric.humidity': 'לחות', 'metric.windSpeed': 'מהירות רוח', 'metric.pressure': 'לחץ',
};

// ═══════════════════════════════════════════════════════════
// ARABIC
// ═══════════════════════════════════════════════════════════
const ar: Translations = {
  'nav.home': 'الرئيسية', 'nav.spec': 'مواصفات G.A.N.E', 'nav.gane': 'G.A.N.E', 'nav.search': 'البحث في المواصفات...',
  'home.title': 'محرك الملاحة المتقدم العالمي', 'home.subtitle': 'مواصفات هندسية شاملة لبنية تحتية لامركزية بدرجة دفاعية لإدارة شبكات المرور العالمية.',
  'home.cta': 'استكشاف المواصفات', 'home.explore': 'استكشاف G.A.N.E',
  'spec.title': 'المواصفات الهندسية', 'spec.search': 'البحث في المواصفات...', 'spec.sections': 'أقسام', 'spec.parts': 'أجزاء',
  'spec.previous': 'السابق', 'spec.next': 'التالي', 'spec.toc': 'جدول المحتويات',
  'gane.title': 'G.A.N.E Navigator', 'gane.subtitle': 'محرك الملاحة المتقدم العالمي — هندسة النظام الكاملة',
  'gane.features': 'الميزات', 'gane.layer': 'طبقة النظام', 'gane.buildOrder': 'ترتيب البناء', 'gane.systemDef': 'تعريف النظام',
  'voice.play': 'قراءة بصوت عالٍ', 'voice.pause': 'إيقاف مؤقت', 'voice.stop': 'إيقاف', 'voice.speed': 'السرعة', 'voice.language': 'لغة الصوت',
  'lang.select': 'اختر اللغة', 'lang.current': 'اللغة الحالية',
  'common.loading': 'جاري التحميل...', 'common.error': 'خطأ', 'common.close': 'إغلاق', 'common.back': 'رجوع',
  'common.readAloud': 'قراءة بصوت عالٍ', 'common.download': 'تحميل', 'common.share': 'مشاركة',
  'common.save': 'حفظ', 'common.cancel': 'إلغاء', 'common.confirm': 'تأكيد', 'common.delete': 'حذف',
  'common.search': 'بحث', 'common.settings': 'الإعدادات', 'common.navigate': 'تنقل',
  'common.on': 'تشغيل', 'common.off': 'إيقاف', 'common.yes': 'نعم', 'common.no': 'لا',
  'stats.parts': 'أجزاء المواصفات', 'stats.sections': 'الأقسام التقنية', 'stats.features': 'ميزات النظام',
  'stats.languages': 'اللغات', 'stats.entities': 'الكيانات الأساسية', 'stats.events': 'أنواع الأحداث', 'stats.phases': 'مراحل البناء',
  'mode.drive': 'قيادة', 'mode.walk': 'مشي', 'mode.emergency': 'طوارئ', 'mode.plan': 'تخطيط',
  'group.nav': 'ملاحة', 'group.intel': 'استخبارات', 'group.ops': 'عمليات', 'group.ai': 'ذكاء', 'group.sys': 'نظام',
  'sidebar.layers': 'طبقات', 'sidebar.parking': 'مواقف', 'sidebar.charging': 'شحن', 'sidebar.transport': 'نقل',
  'sidebar.indoor': 'داخلي', 'sidebar.arNav': 'AR', 'sidebar.history': 'سجل', 'sidebar.draw': 'رسم',
  'sidebar.weather': 'طقس', 'sidebar.satellite': 'قمر صناعي', 'sidebar.imagery': 'صور فضائية', 'sidebar.gnss': 'GNSS',
  'sidebar.v2x': 'V2X', 'sidebar.twin': 'توأم', 'sidebar.risk': 'مخاطر', 'sidebar.coverage': 'تغطية',
  'sidebar.cmd': 'قيادة', 'sidebar.eoc': 'طوارئ', 'sidebar.fleet': 'أسطول', 'sidebar.cameras': 'كاميرات',
  'sidebar.report': 'تقرير', 'sidebar.incident': 'حادث', 'sidebar.alerts': 'تنبيهات', 'sidebar.c4isr': 'قيادة', 'sidebar.radio': 'راديو',
  'sidebar.optimizer': 'محسّن', 'sidebar.score': 'نقاط', 'sidebar.analytics': 'تحليلات', 'sidebar.social': 'اجتماعي',
  'sidebar.liveShare': 'مشاركة', 'sidebar.collab': 'تعاون', 'sidebar.liveTeam': 'فريق مباشر',
  'sidebar.arch': 'هندسة', 'sidebar.pipeline': 'خط أنابيب', 'sidebar.spec': 'مواصفات', 'sidebar.wallet': 'محفظة',
  'sidebar.offline': 'بدون إنترنت', 'sidebar.offlineMaps': 'خرائط بدون إنترنت', 'sidebar.a11y': 'إتاحة', 'sidebar.engine': 'محرك', 'sidebar.battery': 'بطارية',
  'dock.traffic': 'حركة المرور', 'dock.settings': 'الإعدادات', 'dock.admin': 'مدير', 'dock.more': 'المزيد',
  'search.whereTo': 'إلى أين؟', 'search.listening': 'أستمع...', 'search.results': 'النتائج',
  'search.nearby': 'أماكن قريبة', 'search.favorites': 'المفضلة', 'search.recent': 'عمليات بحث حديثة',
  'search.suggestions': 'اقتراحات ذكية',
  'search.food': 'مطاعم', 'search.gas': 'وقود', 'search.coffee': 'قهوة', 'search.parking': 'مواقف',
  'search.shopping': 'تسوق', 'search.hospital': 'مستشفى',
  'search.bestTime': 'أفضل وقت للقيادة للعمل: الآن', 'search.parkingAvailable': 'مواقف متاحة بالقرب من وجهتك الأخيرة',
  'search.goodWeather': 'طقس جيد للمشي',
  'route.drive': 'قيادة', 'route.walk': 'مشي', 'route.emergency': 'طوارئ', 'route.plan': 'تخطيط',
  'route.start': 'بداية', 'route.destination': 'وجهة', 'route.addStop': 'إضافة محطة',
  'route.fastest': 'أسرع', 'route.shortest': 'أقصر', 'route.scenic': 'مناظر',
  'settings.voiceGuidance': 'التوجيه الصوتي', 'settings.alertSounds': 'أصوات التنبيه',
  'settings.hudDisplay': 'شاشة HUD', 'settings.speedDisplay': 'عرض السرعة',
  'settings.animations': 'الرسوم المتحركة', 'settings.offlineMaps': 'خرائط بدون إنترنت',
  'settings.locationSharing': 'مشاركة الموقع', 'settings.language': 'اللغة',
  'settings.theme': 'السمة', 'settings.notifications': 'الإشعارات', 'settings.accessibility': 'إمكانية الوصول',
  'notif.info': 'معلومات', 'notif.success': 'نجاح', 'notif.warning': 'تحذير', 'notif.error': 'خطأ',
  'notif.system': 'نظام', 'notif.collab': 'تعاون', 'notif.admin': 'إدارة',
  'notif.sound': 'أصوات', 'notif.toast': 'نوافذ منبثقة',
  'power.performance': 'أداء', 'power.balanced': 'متوازن', 'power.saver': 'توفير', 'power.ultraSaver': 'توفير فائق',
  'a11y.title': 'إمكانية الوصول', 'a11y.highContrast': 'تباين عالٍ', 'a11y.reduceMotion': 'تقليل الحركة',
  'a11y.fontSize': 'حجم الخط', 'a11y.colorBlind': 'وضع عمى الألوان', 'a11y.screenReader': 'قارئ الشاشة',
  'a11y.skipToContent': 'انتقل إلى المحتوى الرئيسي',
  'a11y.normal': 'عادي', 'a11y.large': 'كبير', 'a11y.extraLarge': 'كبير جداً',
  'ai.greeting': 'مرحباً، أنا G.A.N.E AI', 'ai.subtitle': 'اسألني عن المسارات والمرور والطقس والمواقف...',
  'ai.placeholder': 'اسأل G.A.N.E AI...', 'ai.noInsights': 'لا توجد رؤى حالياً',
  'ai.chat': 'محادثة', 'ai.insights': 'رؤى',
  'incident.accident': 'حادث', 'incident.roadblock': 'حاجز', 'incident.police': 'شرطة',
  'incident.hazard': 'خطر', 'incident.construction': 'أعمال بناء', 'incident.flooding': 'فيضان',
  'incident.speedTrap': 'رادار سرعة', 'incident.roadClosed': 'طريق مغلق',
  'incident.liveFeed': 'بث مباشر', 'incident.report': 'تقرير', 'incident.stats': 'إحصائيات',
  'layer.traffic': 'حركة مرور حية', 'layer.transit': 'نقل عام', 'layer.cycling': 'شبكة دراجات',
  'layer.satellite': 'صور فضائية', 'layer.terrain': 'تضاريس وارتفاع',
  'layer.weatherRadar': 'رادار هطول', 'layer.temperature': 'خريطة حرارة',
  'layer.wind': 'أنماط الرياح', 'layer.precipitation': 'توقعات هطول', 'layer.airQuality': 'مؤشر جودة الهواء',
  'layer.earthquake': 'نشاط زلزالي', 'layer.riskZones': 'مناطق خطر', 'layer.nightVision': 'رؤية ليلية',
  'layer.heatmap': 'خريطة حرارية', 'layer.evCharging': 'شحن كهربائي', 'layer.smartParking': 'مواقف ذكية',
  'layer.buildings3d': 'مباني ثلاثية الأبعاد', 'layer.indoorMaps': 'خرائط داخلية', 'layer.laneGuidance': 'توجيه المسارات',
  'theme.ganeDark': 'G.A.N.E داكن', 'theme.midnight': 'منتصف الليل', 'theme.satellite': 'قمر صناعي',
  'theme.topographic': 'طبوغرافي', 'theme.nightVision': 'رؤية ليلية', 'theme.thermal': 'حراري',
  'layerCat.core': 'ملاحة أساسية', 'layerCat.weather': 'طقس وغلاف جوي',
  'layerCat.safety': 'سلامة ومخاطر', 'layerCat.infrastructure': 'بنية تحتية', 'layerCat.advanced': 'متقدم / تجريبي',
  'transport.drive': 'قيادة', 'transport.transit': 'نقل عام', 'transport.bike': 'دراجة',
  'transport.walk': 'مشي', 'transport.train': 'قطار', 'transport.scooter': 'سكوتر',
  'score.safety': 'سلامة', 'score.alertness': 'يقظة', 'score.smoothness': 'سلاسة', 'score.eco': 'قيادة بيئية',
  'evidence.accident': 'حادث', 'evidence.hazard': 'خطر طريق', 'evidence.police': 'شرطة',
  'evidence.closure': 'طريق مغلق', 'evidence.pothole': 'حفرة', 'evidence.weather': 'طقس',
  'share.viewOnly': 'عرض فقط', 'share.viewEta': 'عرض + وقت الوصول', 'share.fullAccess': 'وصول كامل',
  'severity.critical': 'حرج', 'severity.high': 'عالي', 'severity.medium': 'متوسط', 'severity.low': 'منخفض',
  'alert.green': 'أخضر — عادي', 'alert.yellow': 'أصفر — مرتفع', 'alert.orange': 'برتقالي — طوارئ', 'alert.red': 'أحمر — حرج',
  'boot.quantumCore': 'نواة كمية', 'boot.systemCrates': 'وحدات النظام', 'boot.neuralRouting': 'محرك توجيه عصبي',
  'boot.gnssLock': 'قفل الأقمار الصناعية', 'boot.dataStreams': 'تدفقات بيانات حية', 'boot.holoUI': 'طبقة هولوغرافية',
  'pipeline.weather': 'طقس', 'pipeline.airQuality': 'جودة الهواء', 'pipeline.seismic': 'زلزالي',
  'pipeline.position': 'موقع', 'pipeline.elevation': 'ارتفاع', 'pipeline.maps': 'خرائط',
  'c4isr.fleet': 'أسطول', 'c4isr.missions': 'مهام', 'c4isr.vrp': 'تحسين',
  'analytics.overview': 'نظرة عامة', 'analytics.weather': 'طقس', 'analytics.seismic': 'زلزالي', 'analytics.network': 'شبكة',
  'analytics.title': 'محرك التحليلات', 'analytics.subtitle': 'تحليل بيانات في الوقت الفعلي', 'analytics.systemHealth': 'صحة النظام',
  'error.componentFailed': 'فشل تحميل المكون', 'error.tryAgain': 'حاول مرة أخرى', 'error.goHome': 'العودة للرئيسية',
  'panel.gnssManager': 'مدير GNSS', 'panel.indoorPos': 'تحديد موقع داخلي', 'panel.arNav': 'ملاحة AR', 'panel.riskEngine': 'محرك المخاطر',
  'risk.weather': 'مخاطر الطقس', 'risk.seismic': 'مخاطر زلزالية', 'risk.position': 'مخاطر الموقع',
  'metric.temperature': 'درجة الحرارة', 'metric.humidity': 'الرطوبة', 'metric.windSpeed': 'سرعة الرياح', 'metric.pressure': 'الضغط',
};

// ═══════════════════════════════════════════════════════════
// SPANISH
// ═══════════════════════════════════════════════════════════
const es: Translations = {
  ...en,
  'nav.home': 'Inicio', 'nav.spec': 'Especificación G.A.N.E', 'nav.search': 'Buscar especificación...',
  'home.title': 'Motor de Navegación Avanzado Global', 'home.subtitle': 'Especificación maestra de ingeniería integral para una infraestructura descentralizada de grado de defensa.',
  'home.cta': 'Explorar Especificación', 'home.explore': 'Explorar G.A.N.E',
  'spec.title': 'Especificación de Ingeniería', 'spec.search': 'Buscar...', 'spec.sections': 'secciones', 'spec.parts': 'partes',
  'spec.previous': 'Anterior', 'spec.next': 'Siguiente', 'spec.toc': 'Tabla de Contenidos',
  'common.loading': 'Cargando...', 'common.error': 'Error', 'common.close': 'Cerrar', 'common.back': 'Atrás',
  'common.save': 'Guardar', 'common.cancel': 'Cancelar', 'common.confirm': 'Confirmar', 'common.delete': 'Eliminar',
  'common.search': 'Buscar', 'common.settings': 'Configuración', 'common.navigate': 'Navegar',
  'mode.drive': 'Conducir', 'mode.walk': 'Caminar', 'mode.emergency': 'SOS', 'mode.plan': 'Planificar',
  'group.nav': 'NAV', 'group.intel': 'INTEL', 'group.ops': 'OPS', 'group.ai': 'IA', 'group.sys': 'SIS',
  'dock.traffic': 'Tráfico', 'dock.settings': 'Ajustes', 'dock.admin': 'Admin', 'dock.more': 'Más',
  'search.whereTo': '¿A dónde?', 'search.listening': 'Escuchando...', 'search.results': 'Resultados',
  'search.nearby': 'Lugares cercanos', 'search.favorites': 'Favoritos', 'search.recent': 'Búsquedas recientes',
  'search.suggestions': 'Sugerencias inteligentes',
  'search.food': 'Comida', 'search.gas': 'Gasolina', 'search.coffee': 'Café', 'search.parking': 'Aparcamiento',
  'search.shopping': 'Compras', 'search.hospital': 'Hospital',
  'settings.voiceGuidance': 'Guía de voz', 'settings.alertSounds': 'Sonidos de alerta',
  'a11y.title': 'Accesibilidad', 'a11y.highContrast': 'Alto contraste', 'a11y.reduceMotion': 'Reducir movimiento',
  'ai.greeting': 'Hola, soy G.A.N.E AI', 'ai.placeholder': 'Pregunta a G.A.N.E AI...',
};

// ═══════════════════════════════════════════════════════════
// FRENCH
// ═══════════════════════════════════════════════════════════
const fr: Translations = {
  ...en,
  'nav.home': 'Accueil', 'nav.spec': 'Spécification G.A.N.E', 'nav.search': 'Rechercher...',
  'home.title': 'Moteur de Navigation Avancé Global', 'home.subtitle': 'Spécification maîtresse d\'ingénierie pour une infrastructure décentralisée de grade défense.',
  'home.cta': 'Explorer la Spécification', 'home.explore': 'Explorer G.A.N.E',
  'spec.title': 'Spécification d\'Ingénierie', 'spec.search': 'Rechercher...', 'spec.sections': 'sections', 'spec.parts': 'parties',
  'spec.previous': 'Précédent', 'spec.next': 'Suivant', 'spec.toc': 'Table des Matières',
  'common.loading': 'Chargement...', 'common.error': 'Erreur', 'common.close': 'Fermer', 'common.back': 'Retour',
  'common.save': 'Enregistrer', 'common.cancel': 'Annuler', 'common.confirm': 'Confirmer', 'common.delete': 'Supprimer',
  'common.search': 'Rechercher', 'common.settings': 'Paramètres', 'common.navigate': 'Naviguer',
  'mode.drive': 'Conduire', 'mode.walk': 'Marcher', 'mode.emergency': 'SOS', 'mode.plan': 'Planifier',
  'group.nav': 'NAV', 'group.intel': 'INTEL', 'group.ops': 'OPS', 'group.ai': 'IA', 'group.sys': 'SYS',
  'dock.traffic': 'Trafic', 'dock.settings': 'Paramètres', 'dock.admin': 'Admin', 'dock.more': 'Plus',
  'search.whereTo': 'Où aller ?', 'search.listening': 'Écoute...', 'search.results': 'Résultats',
  'search.nearby': 'Lieux proches', 'search.favorites': 'Favoris', 'search.recent': 'Recherches récentes',
  'settings.voiceGuidance': 'Guidage vocal', 'settings.alertSounds': 'Sons d\'alerte',
  'a11y.title': 'Accessibilité', 'ai.greeting': 'Bonjour, je suis G.A.N.E AI',
};

// ═══════════════════════════════════════════════════════════
// GERMAN
// ═══════════════════════════════════════════════════════════
const de: Translations = {
  ...en,
  'nav.home': 'Startseite', 'nav.spec': 'G.A.N.E-Spezifikation', 'nav.search': 'Spezifikation durchsuchen...',
  'home.title': 'Globaler Fortschrittlicher Navigationsmotor', 'home.subtitle': 'Umfassende technische Masterspezifikation für eine dezentrale Infrastruktur auf Verteidigungsniveau.',
  'home.cta': 'Spezifikation erkunden', 'home.explore': 'G.A.N.E erkunden',
  'common.loading': 'Laden...', 'common.error': 'Fehler', 'common.close': 'Schließen', 'common.back': 'Zurück',
  'common.save': 'Speichern', 'common.cancel': 'Abbrechen', 'common.search': 'Suchen', 'common.settings': 'Einstellungen',
  'mode.drive': 'Fahren', 'mode.walk': 'Gehen', 'mode.emergency': 'SOS', 'mode.plan': 'Planen',
  'dock.traffic': 'Verkehr', 'dock.settings': 'Einstellungen', 'dock.admin': 'Admin', 'dock.more': 'Mehr',
  'search.whereTo': 'Wohin?', 'search.listening': 'Höre zu...', 'search.results': 'Ergebnisse',
  'settings.voiceGuidance': 'Sprachführung', 'a11y.title': 'Barrierefreiheit',
  'ai.greeting': 'Hallo, ich bin G.A.N.E AI',
};

// ═══════════════════════════════════════════════════════════
// RUSSIAN
// ═══════════════════════════════════════════════════════════
const ru: Translations = {
  ...en,
  'nav.home': 'Главная', 'nav.spec': 'Спецификация G.A.N.E', 'nav.search': 'Поиск...',
  'home.title': 'Глобальный Продвинутый Навигационный Движок', 'home.subtitle': 'Комплексная инженерная спецификация для децентрализованной инфраструктуры оборонного класса.',
  'home.cta': 'Изучить Спецификацию', 'home.explore': 'Изучить G.A.N.E',
  'common.loading': 'Загрузка...', 'common.error': 'Ошибка', 'common.close': 'Закрыть', 'common.back': 'Назад',
  'common.save': 'Сохранить', 'common.cancel': 'Отмена', 'common.search': 'Поиск', 'common.settings': 'Настройки',
  'mode.drive': 'Вождение', 'mode.walk': 'Пешком', 'mode.emergency': 'SOS', 'mode.plan': 'Планирование',
  'dock.traffic': 'Трафик', 'dock.settings': 'Настройки', 'dock.admin': 'Админ', 'dock.more': 'Ещё',
  'search.whereTo': 'Куда?', 'search.listening': 'Слушаю...', 'search.results': 'Результаты',
  'settings.voiceGuidance': 'Голосовые подсказки', 'a11y.title': 'Доступность',
  'ai.greeting': 'Привет, я G.A.N.E AI',
};

// ═══════════════════════════════════════════════════════════
// CHINESE
// ═══════════════════════════════════════════════════════════
const zh: Translations = {
  ...en,
  'nav.home': '首页', 'nav.spec': 'G.A.N.E规范', 'nav.search': '搜索规范...',
  'home.title': '全球先进导航引擎', 'home.subtitle': '去中心化、国防级基础设施的综合工程总规范，用于管理大陆级全球交通网络。',
  'home.cta': '探索规范', 'home.explore': '探索 G.A.N.E',
  'common.loading': '加载中...', 'common.error': '错误', 'common.close': '关闭', 'common.back': '返回',
  'common.save': '保存', 'common.cancel': '取消', 'common.search': '搜索', 'common.settings': '设置',
  'mode.drive': '驾驶', 'mode.walk': '步行', 'mode.emergency': '紧急', 'mode.plan': '规划',
  'dock.traffic': '交通', 'dock.settings': '设置', 'dock.admin': '管理', 'dock.more': '更多',
  'search.whereTo': '去哪里？', 'search.listening': '正在听...', 'search.results': '结果',
  'settings.voiceGuidance': '语音导航', 'a11y.title': '无障碍',
  'ai.greeting': '你好，我是 G.A.N.E AI',
};

// ═══════════════════════════════════════════════════════════
// TRANSLATIONS MAP
// ═══════════════════════════════════════════════════════════
const translations: Record<string, Translations> = {
  en, he, ar, es, fr, de, ru, zh,
};

// Fallback to English for any missing language
export function t(key: TranslationKey, lang: string): string {
  const langTranslations = translations[lang] || translations['en'];
  return langTranslations[key] || translations['en'][key] || key;
}

export function getLanguage(code: string): Language {
  return languages.find(l => l.code === code) || languages[0];
}

export function isRTL(code: string): boolean {
  const lang = getLanguage(code);
  return lang.dir === 'rtl';
}
