// Mirrors src-tauri/src/models/*.rs — keep in sync when Rust structs change.

export interface SystemInfo {
  device_name: string;
  os: string;
  os_version: string;
  hostname: string;
  cpu: string;
  ram_gb: number;
  disk_gb: number;
}

export interface GitInfo {
  version: string | null;
  gitconfig: string | null;
}

export interface NodeInfo {
  node_version: string | null;
  npm_version: string | null;
  global_packages: string[];
}

export interface PythonInfo {
  python_version: string | null;
  pip_version: string | null;
  packages: string[];
}

export interface DockerInfo {
  docker_version: string | null;
  compose_version: string | null;
}

export interface Snapshot extends SystemInfo {
  applications: string[];
  brew_packages: string[];
  brew_casks: string[];
  vscode_extensions: string[];
  git: GitInfo;
  node: NodeInfo;
  python: PythonInfo;
  docker: DockerInfo;
  configs: string[];
  created_at: string;
}

export interface Device {
  id: string;
  user_id: string;

  device_name: string;
  hostname: string;

  os: string;
  os_version: string;

  cpu: string;
  ram_gb: number;
  disk_gb: number;
}
export interface User {
  id: string;
  email: string;
}

export interface AuthResponse {
  user_id: string;
  email: string;
  access_token: string;
}

export interface RestoreProgress {
  step: string;
  message: string;
  done: boolean;
}
