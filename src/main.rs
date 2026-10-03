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

async fn apply_layout(instance: &Instance) -> OpenActionResult<()> {
	if instance.controller == "Neo" || instance.controller == "Infobar" {
		let _ = instance
			.set_feedback_layout("layouts/counter_neo.json".to_string())
			.await;
	} else if instance.controller == "Encoder" {
		let _ = instance
			.set_feedback_layout("layouts/counter_dial.json".to_string())
			.await;
	}
	Ok(())
}

async fn update_display(instance: &Instance, value: isize) -> OpenActionResult<()> {
	let _ = instance
		.set_title(Some(value.to_string()), None)
		.await;
	if instance.controller != "Keypad" {
		let _ = instance
			.set_feedback(&serde_json::json!({ "value": value.to_string() }))
			.await;
	}
	Ok(())
}

async fn increment(
	instance: &Instance,
	settings: &CounterSettings,
	step: isize,
) -> OpenActionResult<()> {
	let mut clone = settings.clone();
	clone.value = settings.value + step;
	instance.set_settings(&clone).await?;
	update_display(instance, clone.value).await
}

struct PersistedCounterAction;
#[async_trait]
impl Action for PersistedCounterAction {
	const UUID: ActionUuid = "me.amankhanna.oacounter.persisted";
	type Settings = CounterSettings;

	async fn will_appear(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
	) -> OpenActionResult<()> {
		apply_layout(instance).await?;
		update_display(instance, settings.value).await
	}

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
	}

	async fn dial_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
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

	async fn touch_tap(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
		_position: (u16, u16),
		_hold: bool,
	) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
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
		apply_layout(instance).await?;
		increment(instance, settings, -settings.value).await
	}

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
	}

	async fn dial_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
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

	async fn touch_tap(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
		_position: (u16, u16),
		_hold: bool,
	) -> OpenActionResult<()> {
		increment(instance, settings, settings.step).await
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
