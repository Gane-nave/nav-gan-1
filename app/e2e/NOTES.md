# E2E Test Notes

The screenshot shows a blank white page. This means the React app's boot sequence
(QuantumBoot animation) renders on a dark background (#010206) but in headless Chromium
the canvas/animations may not render, and the page appears blank white until boot completes.

The issue is that many tests wait only 3-6 seconds but the boot sequence takes longer
in headless mode. Tests that check for DOM content after boot fail because the boot
animation hasn't completed yet.

Solution: Tests should either:
1. Wait for the boot to complete by checking for post-boot content
2. Skip the boot by injecting localStorage to mark boot as completed
3. Use longer timeouts
4. Focus on API/server tests that don't require the full UI boot
