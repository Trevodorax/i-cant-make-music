import type { FC } from 'react'
import './App.css'
import { Track } from './components/Track'
import { BpmInput } from './components/BpmInput'

export const App: FC = () => {
  return (
    <div>
      <BpmInput />
      <Track />
    </div>
  )
}
