import type { FC } from 'react'
import { useState } from 'react'
import { useAppState, useUpdateBpm } from '../hooks/useMusic'

export const BpmInput: FC = () => {
  const { data, isLoading } = useAppState()
  const updateBpmMutation = useUpdateBpm()
  const [localBpm, setLocalBpm] = useState<number | undefined>(undefined)

  const currentBpm = localBpm ?? data?.state.bpm ?? 120

  const handleBpmChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const newBpm = parseInt(event.target.value, 10)
    if (!isNaN(newBpm) && newBpm >= 20 && newBpm <= 300) {
      setLocalBpm(newBpm)
      updateBpmMutation.mutate({ bpm: newBpm })
    }
  }

  if (isLoading) return <div>Loading BPM...</div>

  return (
    <div>
      <label htmlFor="bpm-input">BPM: </label>
      <input
        id="bpm-input"
        type="range"
        value={currentBpm}
        onChange={handleBpmChange}
        min="20"
        max="300"
      />
      <span>{currentBpm}</span>
    </div>
  )
}