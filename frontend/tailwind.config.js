/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      colors: {
        // Industrial Backgrounds
        "industrial-slate": "#1B1B1E", // Main Background
        "industrial-surface": "#2A2A2E", // Panel Backgrounds
        "coal-dark": "#1A1A1A", // Darkest background
        "iron-gray": "#2D2D2D", // Mid-tone panels

        // Purdue Gold Variants
        "purdue-gold": "#CFB991", // Base Gold (Metallic)
        "purdue-prime": "#FFD700", // Highlight Gold (Bright)
        "old-gold": "#CFB991", // Official Purdue Old Gold
        "aged-gold": "#8E6F3E", // Darker gold for depth
        "dust-gold": "#EBD99F", // Lighter gold for highlights

        // Core Colors
        "purdue-black": "#000000",
        "purdue-dark": "#1a1a1a",
        "steam-white": "#F0F0F0", // Primary text color
        "purdue-dust": "#a0a0a0", // Secondary text

        // Signal Colors
        "signal-red": "#C8102E", // Errors/warnings
        "gauge-green": "#4F7942", // Success states
        "blueprint-blue": "#1B2A41",
      },
      animation: {
        blob: "blob 7s infinite",
        "fade-in": "fadeIn 0.5s ease-out forwards",
        "slide-in-right": "slideInRight 0.3s ease-out forwards",
        "slide-in-left": "slideInLeft 0.3s ease-out forwards",
        "pulse-gold": "pulseGold 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "glow": "glow 2s ease-in-out infinite",
      },
      keyframes: {
        blob: {
          "0%": { transform: "translate(0px, 0px) scale(1)" },
          "33%": { transform: "translate(30px, -50px) scale(1.1)" },
          "66%": { transform: "translate(-20px, 20px) scale(0.9)" },
          "100%": { transform: "translate(0px, 0px) scale(1)" },
        },
        fadeIn: {
          "0%": { opacity: "0", transform: "translateY(10px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
        slideInRight: {
          "0%": { transform: "translateX(100%)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        slideInLeft: {
          "0%": { transform: "translateX(-100%)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        pulseGold: {
          "0%, 100%": { opacity: "1" },
          "50%": { opacity: "0.7" },
        },
        glow: {
          "0%, 100%": { boxShadow: "0 0 20px rgba(207, 185, 145, 0.3)" },
          "50%": { boxShadow: "0 0 30px rgba(207, 185, 145, 0.6)" },
        },
      },
      backgroundImage: {
        "blueprint-grid": "linear-gradient(to right, #334155 1px, transparent 1px), linear-gradient(to bottom, #334155 1px, transparent 1px)",
        "metallic-gold": "linear-gradient(135deg, #8E6F3E 0%, #CFB991 40%, #EBD99F 50%, #CFB991 60%, #8E6F3E 100%)",
      },
      backgroundSize: {
        "blueprint-grid": "40px 40px",
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'Roboto Mono', 'Consolas', 'monospace'],
        ui: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        ai: ['JetBrains Mono', 'Roboto Mono', 'Consolas', 'monospace'],
      },
    },
  },
  plugins: [],
}
