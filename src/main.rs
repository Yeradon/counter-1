use openaction::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct CounterSettings {
	step: isize,
	value: isize,
}
impl Default for CounterSettings {
	fn default() -> Self {
		Self { step: 1, value: 0 }
	}
}

async fn increment(
	instance: &Instance,
	settings: &CounterSettings,
	step: isize,
) -> OpenActionResult<()> {
	let mut clone = settings.clone();
	clone.value = settings.value + step;
	instance.set_settings(&clone).await?;
	instance
		.set_title(Some(clone.value.to_string()), None)
		.await
}

struct PersistedCounterAction;
#[async_trait]
impl Action for PersistedCounterAction {
	const UUID: ActionUuid = "me.amankhanna.oacounter.persisted";
	type Settings = CounterSettings;

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
	}

	async fn dial_up(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
	) -> OpenActionResult<()> {
		self.key_down(instance, settings).await
	}

	async fn dial_rotate(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
		increment(instance, settings, settings.step * (ticks as isize)).await
	}
}

struct TemporaryCounterAction;
#[async_trait]
impl Action for TemporaryCounterAction {
	const UUID: ActionUuid = "me.amankhanna.oacounter.temporary";
	type Settings = CounterSettings;

	async fn will_appear(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
	) -> OpenActionResult<()> {
		increment(instance, settings, -settings.value).await
	}

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
	}

	async fn dial_up(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
	) -> OpenActionResult<()> {
		self.key_down(instance, settings).await
	}

	async fn dial_rotate(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
		increment(instance, settings, settings.step * (ticks as isize)).await
	}
}

#[tokio::main]
async fn main() -> OpenActionResult<()> {
	{
		use simplelog::*;
		if let Err(error) = TermLogger::init(
			LevelFilter::Debug,
			Config::default(),
			TerminalMode::Stdout,
			ColorChoice::Never,
		) {
			eprintln!("Logger initialization failed: {}", error);
		}
	}

	register_action(PersistedCounterAction).await;
	register_action(TemporaryCounterAction).await;

	run(std::env::args().collect()).await
}
