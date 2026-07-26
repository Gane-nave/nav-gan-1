import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.gane.nav',
  appName: 'G.A.N.E Navigator',
  webDir: 'www',
  backgroundColor: '#03060C',
  server: {
    androidScheme: 'https',
    cleartext: false
  },
  android: {
    allowMixedContent: false,
    captureInput: true,
    webContentsDebuggingEnabled: true,
    backgroundColor: '#03060C'
  },
  plugins: {
    Geolocation: {
      permissions: ['location']
    },
    SplashScreen: {
      launchShowDuration: 1500,
      backgroundColor: '#03060C',
      showSpinner: false
    }
  }
};

export default config;
