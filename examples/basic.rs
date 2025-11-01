// use kobe_client::client::KobeClient;
//
// #[tokio::main]
// async fn main() {
//     // Create a client with mainnet defaults
//     let client = KobeClient::mainnet();
//
//     println!("=== Jito API Client - Basic Example ===\n");
//
//     // 2. Get staker rewards
//     println!("2. Getting staker rewards (limit 5)...");
//     let rewards = client.get_staker_rewards(Some(5)).await.unwrap();
//     println!("   Found {} staker rewards:", rewards.rewards.len());
//     for (i, reward) in rewards.rewards.iter().enumerate() {
//         println!(
//             "   {}. Stake Account: {}",
//             i + 1,
//             &reward.stake_account[..8]
//         );
//         println!("      MEV Rewards: {} lamports", reward.mev_rewards);
//         println!("      Epoch: {}", reward.epoch);
//         println!("      Claimed: {}", reward.mev_claimed);
//     }
//     println!();
//
//     // 3. Get validator information
//     println!("3. Getting validators running Jito...");
//     let jito_validators = client.get_jito_validators().await.unwrap();
//     println!("   Found {} validators running Jito", jito_validators.len());
//
//     // Show top 5 by stake
//     let top_5_by_stake = jito_validators.iter().take(5);
//     for (i, validator) in top_5_by_stake.enumerate() {
//         println!(
//             "   {}. Vote Account: {}",
//             i + 1,
//             &validator.vote_account[..8]
//         );
//         println!("      Active Stake: {} lamports", validator.active_stake);
//         println!("      MEV Commission: {} bps", validator.mev_commission_bps);
//     }
//     println!();
//
//     // 4. Get network MEV statistics
//     println!("4. Getting MEV network statistics...");
//     let mev_stats = client.get_mev_rewards(None).await.unwrap();
//     println!("   Epoch: {}", mev_stats.epoch);
//     println!(
//         "   Total Network MEV: {} lamports",
//         mev_stats.total_network_mev_lamports
//     );
//     println!(
//         "   Jito Stake Weight: {} lamports",
//         mev_stats.jito_stake_weight_lamports
//     );
//     println!(
//         "   MEV per lamport: {:.10}",
//         mev_stats.mev_reward_per_lamport
//     );
//     println!();
//
//     // 5. Get validator rewards
//     println!("5. Getting validator rewards...");
//     let validator_rewards = client.get_validator_rewards(None, Some(3)).await.unwrap();
//     println!("   Top 3 validators by rewards:");
//     for (i, validator) in validator_rewards.validators.iter().enumerate() {
//         println!(
//             "   {}. Vote Account: {}",
//             i + 1,
//             &validator.vote_account[..8]
//         );
//         println!("      MEV Rewards: {} lamports", validator.mev_rewards);
//         if let Some(pf_rewards) = validator.priority_fee_rewards {
//             println!("      Priority Fee Rewards: {} lamports", pf_rewards);
//         }
//     }
//     println!();
//
//     // 6. Get historical validator data
//     println!("6. Getting historical data for a validator...");
//     if let Some(first_validator) = jito_validators.first() {
//         let history = client
//             .get_validator_history(&first_validator.vote_account)
//             .await
//             .unwrap();
//         println!("   Validator: {}", &first_validator.vote_account[..8]);
//         println!("   Historical data (last 3 epochs):");
//         for (i, entry) in history.iter().take(3).enumerate() {
//             println!(
//                 "   {}. Epoch {}: {} lamports",
//                 i + 1,
//                 entry.epoch,
//                 entry.mev_rewards
//             );
//         }
//     }
//     println!();
//
//     // 7. Get MEV commission averages
//     println!("7. Getting MEV commission averages...");
//     let commission_avg = client.get_mev_commission_average_over_time().await.unwrap();
//     println!(
//         "   Aggregated MEV Rewards: {} lamports",
//         commission_avg.aggregated_mev_rewards
//     );
//     if let Some(latest_apy) = commission_avg.apy.first() {
//         println!("   Latest APY: {:.2}%", latest_apy.data * 100.0);
//     }
//     if let Some(latest_tvl) = commission_avg.tvl.first() {
//         println!("   Latest TVL: {} lamports", latest_tvl.data);
//     }
//     println!();
//
//     // 8. Check specific validator
//     println!("8. Checking specific validator status...");
//     let test_vote_account = "GdRKUZKdiXMEATjddQW6q4W8bPgXRBYJKayfeqdQcEPa";
//     let is_running_jito = client
//         .is_validator_running_jito(test_vote_account)
//         .await
//         .unwrap();
//     println!("   Vote Account: {}", &test_vote_account[..8]);
//     println!("   Running Jito: {}", is_running_jito);
//
//     if let Some(commission) = client
//         .get_validator_mev_commission(test_vote_account)
//         .await
//         .unwrap()
//     {
//         println!("   MEV Commission: {} bps", commission);
//     }
//     println!();
//
//     // 9. Calculate total MEV for epoch range
//     println!("9. Calculating total MEV rewards...");
//     let start_epoch = current_epoch.saturating_sub(5);
//     let end_epoch = current_epoch.saturating_sub(1);
//     let total_mev = client
//         .calculate_total_mev_rewards(start_epoch, end_epoch)
//         .await
//         .unwrap();
//     println!(
//         "   Total MEV from epoch {} to {}: {} lamports",
//         start_epoch, end_epoch, total_mev
//     );
//     println!();
//
//     println!("=== Example completed successfully! ===");
// }
//
