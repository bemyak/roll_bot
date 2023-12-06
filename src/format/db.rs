use std::collections::HashMap;

use crate::get_unix_time;

pub fn format_collection_metadata(meta: ejdb::meta::DatabaseMetadata) -> String {
	meta.collections()
		.map(|col| {
			format!(
				"<code>{}</code>: <code>{}</code> records",
				col.name(),
				col.records()
			)
		})
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn format_message_stats(msgs: Vec<LogMessage>) -> Result<String, ejdb::Error> {
	let now = get_unix_time();
	let mount_ago = now - 60 * 60 * 24 * 30;

	let msg_total = msgs.len();
	let msg_total_month = msgs.iter().filter(|msg| msg.timestamp >= mount_ago).count();

	let max_latency = msgs.iter().map(|msg| msg.latency).max().unwrap_or_default() as usize;
	let max_latency_month = msgs
		.iter()
		.filter(|msg| msg.timestamp >= mount_ago)
		.map(|msg| msg.latency)
		.max()
		.unwrap_or_default() as usize;

	let min_latency = msgs.iter().map(|msg| msg.latency).min().unwrap_or_default() as usize;
	let min_latency_month = msgs
		.iter()
		.filter(|msg| msg.timestamp >= mount_ago)
		.map(|msg| msg.latency)
		.min()
		.unwrap_or_default() as usize;

	let avg_latency = match msg_total {
		0 => 0,
		_ => msgs.iter().fold(0, |acc, msg| acc + msg.latency) as usize / msg_total,
	};

	let avg_latency_month = match msg_total_month {
		0 => 0,
		_ => {
			msgs.iter()
				.filter(|msg| msg.timestamp >= mount_ago)
				.fold(0, |acc, msg| acc + msg.latency) as usize
				/ msg_total_month
		}
	};

	let users: HashMap<i64, u64> = {
		let mut users = HashMap::new();

		msgs.iter().for_each(|msg| {
			let old_ts = users.get(&msg.user_id);
			match old_ts {
				None => {
					users.insert(msg.user_id, msg.timestamp);
				}
				Some(old_ts) => {
					if old_ts < &msg.timestamp {
						users.insert(msg.user_id, msg.timestamp);
					}
				}
			}
		});
		users
	};

	let users_total = users.len();
	let users_total_month = users.iter().filter(|(_, ts)| ts >= &&mount_ago).count();

	Ok(format!(
		"Total messages: <code>{msg_total_month}</code> / <code>{msg_total}</code>
Unique users: <code>{users_total_month}</code> / <code>{users_total}</code>
Max latency, ms: <code>{max_latency_month}</code> / <code>{max_latency}</code>
Avg latency, ms: <code>{avg_latency_month}</code> / <code>{avg_latency}</code>
Min latency, ms: <code>{min_latency_month}</code> / <code>{min_latency}</code>",
	))
}
