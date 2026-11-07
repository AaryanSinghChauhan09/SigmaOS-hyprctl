//! hyprctl model — Hyprland compositor control.

#[derive(Debug, Clone)]
pub struct Client {
    pub address: String,
    pub class: String,
    pub title: String,
    pub workspace: i32,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub clients: Vec<Client>,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: i32,
}

impl Default for Model {
    fn default() -> Self { Self {
        clients: vec![Client { address: "0x1234".into(), class: "alacritty".into(), title: "Terminal".into(), workspace: 1 }],
        workspaces: vec![Workspace { id: 1, name: "1".into() }, Workspace { id: 2, name: "2".into() }],
        active_workspace: 1,
    } }
}

impl Model {
    pub fn new() -> Self { Self::default() }
    pub fn list(&self) -> &[Client] { &self.clients }
    pub fn clients_on(&self, ws: i32) -> Vec<&Client> {
        self.clients.iter().filter(|c| c.workspace == ws).collect()
    }
    pub fn dispatch(&mut self, cmd: &str) -> bool {
        !cmd.is_empty()
    }
    pub fn active(&self) -> i32 { self.active_workspace }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clients_workspace() {
        let mut h = Model::new();
        assert_eq!(h.clients_on(1).len(), 1);
        assert!(h.dispatch("workspace 2"));
    }
}
