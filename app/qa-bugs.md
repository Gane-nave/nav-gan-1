# Critical Bugs Found During QA

## BUG 1: Settings button doesn't close A11y panel
- **Steps**: Open A11y panel → Click "Settings" in bottom dock
- **Expected**: A11y panel closes, Settings view opens
- **Actual**: A11y panel stays open, Settings doesn't open
- **Root cause**: SET_VIEW now closes activePanel (we fixed this), but the bottom dock Settings button might be using a different action
- **Fix needed**: Check how bottom dock Settings button dispatches — it should close the sidebar panel AND switch view

## BUG 2: FPS still shows 20-26fps
- **Expected**: Should show closer to 60fps since we optimized canvas
- **Root cause**: The FPS counter itself may be inaccurate, or Google Maps rendering is heavy

## All panels tested and working:
- Layers ✅, Parking ✅, Charging ✅, Transport ✅, Indoor ✅, AR Nav ✅
- Weather ✅, GNSS ✅, Risk ✅, CMD ✅, EOC ✅, Optimizer ✅
- Score ✅, Social ✅, Arch ✅, Wallet ✅, Offline ✅, A11y ✅
