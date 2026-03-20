/// Distribution channel management: OEM, pre-install, SDK, app store.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelType { OemPreInstall, CarrierBundle, AppStore, SdkEmbed, WebApp, AndroidAuto, CarPlay, HeadUnit }
#[derive(Debug, Clone)]
pub struct DistributionChannel { pub channel_type: ChannelType, pub partner_name: String, pub estimated_reach: u64, pub conversion_rate: f64, pub active: bool }
impl DistributionChannel {
    pub fn projected_users(&self) -> u64 { if !self.active { 0 } else { (self.estimated_reach as f64 * self.conversion_rate) as u64 } }
}
#[derive(Debug, Clone)]
pub struct DistributionManager { pub channels: Vec<DistributionChannel> }
impl Default for DistributionManager {
    fn default() -> Self { Self::new() }
}
impl DistributionManager {
    pub fn new() -> Self { Self { channels: Vec::new() } }
    pub fn add_channel(&mut self, ch: DistributionChannel) { self.channels.push(ch); }
    pub fn total_projected_users(&self) -> u64 { self.channels.iter().map(|c| c.projected_users()).sum() }
    pub fn active_channels(&self) -> usize { self.channels.iter().filter(|c| c.active).count() }
    pub fn top_channels(&self, n: usize) -> Vec<&DistributionChannel> {
        let mut s: Vec<_> = self.channels.iter().collect();
        s.sort_by_key(|b| std::cmp::Reverse(b.projected_users()));
        s.into_iter().take(n).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_projected() { let c = DistributionChannel { channel_type: ChannelType::OemPreInstall, partner_name: "T".into(), estimated_reach: 1_000_000, conversion_rate: 0.3, active: true }; assert_eq!(c.projected_users(), 300_000); }
    #[test] fn test_inactive() { let c = DistributionChannel { channel_type: ChannelType::AppStore, partner_name: "G".into(), estimated_reach: 5_000_000, conversion_rate: 0.1, active: false }; assert_eq!(c.projected_users(), 0); }
    #[test] fn test_mgr() { let mut m = DistributionManager::new(); m.add_channel(DistributionChannel { channel_type: ChannelType::OemPreInstall, partner_name: "A".into(), estimated_reach: 100_000, conversion_rate: 0.5, active: true }); assert_eq!(m.active_channels(), 1); }
    #[test] fn test_top() { let mut m = DistributionManager::new(); m.add_channel(DistributionChannel { channel_type: ChannelType::SdkEmbed, partner_name: "S".into(), estimated_reach: 1000, conversion_rate: 0.5, active: true }); m.add_channel(DistributionChannel { channel_type: ChannelType::OemPreInstall, partner_name: "B".into(), estimated_reach: 1_000_000, conversion_rate: 0.5, active: true }); assert_eq!(m.top_channels(1)[0].partner_name, "B"); }
}
