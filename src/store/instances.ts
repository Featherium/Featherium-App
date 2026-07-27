import { create } from "zustand"

import {
  closeServiceInstance,
  focusServiceInstance,
  listServiceInstances,
  openServiceInstance,
  setInstanceUserAgent,
} from "@/lib/tauri-commands"

export interface ServiceInstance {
  id: string
  recipeId: string
  label: string
  nativeUserAgent: boolean
}

interface InstancesState {
  instances: ServiceInstance[]
  activeInstanceId: string | null
  hydrated: boolean
  hydrate: () => Promise<void>
  openInstance: (recipeId: string, label: string) => Promise<void>
  focusInstance: (id: string) => Promise<void>
  closeInstance: (id: string) => Promise<void>
  setUserAgent: (id: string, native: boolean) => Promise<void>
}

export const useInstancesStore = create<InstancesState>((set, get) => ({
  instances: [],
  activeInstanceId: null,
  hydrated: false,

  hydrate: async () => {
    const summaries = await listServiceInstances()
    set({
      instances: summaries.map((summary) => ({
        id: summary.id,
        recipeId: summary.recipeId,
        label: summary.label,
        nativeUserAgent: summary.nativeUserAgent,
      })),
      activeInstanceId: summaries[0]?.id ?? null,
      hydrated: true,
    })
  },

  openInstance: async (recipeId, label) => {
    const id = await openServiceInstance(recipeId, label)
    set((state) => ({
      instances: [...state.instances, { id, recipeId, label, nativeUserAgent: false }],
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

  setUserAgent: async (id, native) => {
    await setInstanceUserAgent(id, native)
    set((state) => ({
      instances: state.instances.map((instance) =>
        instance.id === id ? { ...instance, nativeUserAgent: native } : instance,
      ),
    }))
  },
}))
