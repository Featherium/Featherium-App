import { invoke } from "@tauri-apps/api/core"

export interface ViewportRect {
  x: number
  y: number
  width: number
  height: number
}

export interface InstanceSummary {
  id: string
  recipeId: string
  label: string
  nativeUserAgent: boolean
}

export async function openServiceInstance(recipeId: string, label: string): Promise<string> {
  return invoke("open_service_instance", { recipeId, label })
}

export async function focusServiceInstance(id: string): Promise<void> {
  return invoke("focus_service_instance", { id })
}

export async function closeServiceInstance(id: string): Promise<void> {
  return invoke("close_service_instance", { id })
}

export async function resizeServiceInstance(id: string, bounds: ViewportRect): Promise<void> {
  return invoke("resize_service_instance", { id, bounds })
}

export async function debugReadInstanceMarker(id: string): Promise<string | null> {
  return invoke("debug_read_instance_marker", { id })
}

export async function setInstanceUserAgent(id: string, native: boolean): Promise<void> {
  return invoke("set_instance_user_agent", { id, native })
}

export async function listServiceInstances(): Promise<InstanceSummary[]> {
  return invoke("list_service_instances")
}
