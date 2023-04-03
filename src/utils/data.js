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
export const initDataCheck = async () => {
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

export const saveData = async (data) => {
  if (typeof data !== 'object' || data === null) return false
  await writeTextFile(
    { contents: JSON.stringify(data), path: filePath },
    { dir: baseDir }
  )
  return true
}
