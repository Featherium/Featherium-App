import { create } from "zustand"

import {
  closeServiceInstance,
  focusServiceInstance,
  openServiceInstance,
} from "@/lib/tauri-commands"

export interface ServiceInstance {
  id: string
  recipeId: string
  label: string
}

interface InstancesState {
  instances: ServiceInstance[]
  activeInstanceId: string | null
  openInstance: (recipeId: string, label: string) => Promise<void>
  focusInstance: (id: string) => Promise<void>
  closeInstance: (id: string) => Promise<void>
}

export const useInstancesStore = create<InstancesState>((set, get) => ({
  instances: [],
  activeInstanceId: null,

  openInstance: async (recipeId, label) => {
    const id = await openServiceInstance(recipeId, label)
    set((state) => ({
      instances: [...state.instances, { id, recipeId, label }],
      activeInstanceId: id,
    }))
  },

  focusInstance: async (id) => {
    await focusServiceInstance(id)
    set({ activeInstanceId: id })
  },

  closeInstance: async (id) => {
    await closeServiceInstance(id)
    const { instances, activeInstanceId } = get()
    const remaining = instances.filter((instance) => instance.id !== id)
    set({
      instances: remaining,
      activeInstanceId: activeInstanceId === id ? (remaining[0]?.id ?? null) : activeInstanceId,
    })
  },
}))
