#[derive(Clone)]
pub struct TopicFormatter {
    hass_domain: String,
    addon_domain: String,
}

pub struct Topic {
    pub domain: String,
    pub component: String,
    pub object_id: ObjectId,
    pub suffix: String,
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.domain, self.component, self.object_id, self.suffix
        )
    }
}

pub fn topic_fmt(domain: &str, component: &str, object_id: &ObjectId, suffix: &str) -> String {
    format!("{}/{}/{}/{}", domain, component, object_id, suffix)
}

pub fn topic_defmt(topic: &str) -> Option<Topic> {
    let parts: Vec<_> = topic.split("/").collect();
    if parts.len() != 4 {
        return None;
    }

    Some(Topic {
        domain: parts[0].to_owned(),
        component: parts[1].to_owned(),
        object_id: ObjectId::from_str(parts[2]).ok()?,
        suffix: parts[3].to_owned(),
    })
}

impl TopicFormatter {
    pub fn new(hass_domain: String, addon_domain: String) -> Self {
        TopicFormatter {
            hass_domain,
            addon_domain,
        }
    }

    pub fn hass_discovery_topic(&self, component: &str, object_id: &ObjectId) -> String {
        topic_fmt(&self.hass_domain, component, object_id, "config")
    }

    pub fn sensor_state_topic(&self, object_id: &ObjectId) -> String {
        topic_fmt(&self.addon_domain, "sensor", object_id, "state")
    }

    pub fn button_cmd_topic(&self, object_id: &ObjectId) -> String {
        topic_fmt(&self.addon_domain, "button", object_id, "cmd")
    }

    pub fn number_cmd_topic(&self, object_id: &ObjectId) -> String {
        topic_fmt(&self.addon_domain, "number", object_id, "cmd")
    }

    pub fn switch_cmd_topic(&self, object_id: &ObjectId) -> String {
        topic_fmt(&self.addon_domain, "switch", object_id, "cmd")
    }
}

use super::ObjectId;
use std::{fmt, str::FromStr};
