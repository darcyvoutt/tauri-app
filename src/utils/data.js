import {
  BaseDirectory,
  createDir,
  readDir,
  readTextFile,
  writeTextFile,
} from '@tauri-apps/api/fs'

// Local Variables
const baseDir = BaseDirectory.AppData
const dataDir = 'data'
const dataFile = 'settings.json'
const filePath = `${dataDir}/${dataFile}`

// Local Method
const _createDataFile = async () => {
  try {
    await createDir(dataDir, { dir: baseDir, recursive: true })
    await writeTextFile({ contents: '{}', path: filePath }, { dir: baseDir })
  } catch (e) {
    console.error(e)
  }
}

const _findDataFile = async () => {
  try {
    const files = await readDir(dataDir, { dir: baseDir })
    const fileNames = files.map(({ name }) => name)
    return fileNames.includes(dataFile)
  } catch (e) {
    return false
  }
}

// Export Functions
export const initData = async () => {
  const hasDataFolder = await _findDataFile()
  if (!hasDataFolder) await _createDataFile()
}

export const getData = async () => {
  try {
    const data = await readTextFile(filePath, { dir: baseDir })
    return JSON.parse(data)
  } catch (e) {
    return null
  }
}

export const resetData = async () => {
  try {
    await writeTextFile({ contents: '{}', path: filePath }, { dir: baseDir })
  } catch (e) {
    console.error(e)
  }
}

export const saveData = async (obj) => {
  if (typeof obj !== 'object' || obj === null) return
  await writeTextFile(
    { contents: JSON.stringify(obj), path: filePath },
    { dir: baseDir }
  )
  return true
}
