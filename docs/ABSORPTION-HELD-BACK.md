# Held back from the cross-repo absorption (PR #101)

Four files from `devin/canonical-rename-pass-2` were ported, evaluated, and then
dropped rather than committed:

- `client/src/hooks/useCollabSessionKey.ts`
- `client/src/hooks/useConstellationHandoffAlerts.ts`
- `client/src/hooks/useFallbackTransitionAlerts.ts`
- `client/src/components/FallbackSeverityIndicator.tsx`

## Why they were dropped, not committed

They form a **closed loop that nothing outside reaches**: the indicator imports
the two alert hooks, one hook references the indicator, and `useCollabSessionKey`
has zero importers. No page, component, or router touches any of them.

Wiring the indicator is not a mount-point problem. Its data source —
`client/src/engine/positionFallbackChain.ts` — is **never started**: the only
reference to it in the whole tree is the unused `engine/index.ts` barrel. Mounting
the indicator today would render a permanently empty widget.

## What wiring it would actually take

1. Start `positionFallbackChain` in `contexts/GANEContext.tsx` alongside the other
   engines that *are* driven, and feed it the position stream.
2. Emit fallback-transition events it can observe (the chain has the transition
   model; nothing publishes it).
3. Then mount `FallbackSeverityIndicator` as a HUD overlay in `pages/Home.tsx`.

Worth doing — showing the user when positioning degrades is precisely the
product's stated differentiator ("GNSS is one signal among many … never fails
silently"). It is a feature task, not an absorption task, so it does not belong
in a test-absorption PR.

The files remain recoverable from `refs/devin-pass2` (branch
`devin/canonical-rename-pass-2` of `Gane-nave/Gane--by-DEVIN`).
