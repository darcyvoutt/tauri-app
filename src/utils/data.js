import {
  BaseDirectory,
  createDir,
  readDir,
  readTextFile,
  writeFile,
} from '@tauri-apps/api/fs'

// Local Variables
const baseDir = BaseDirectory.AppData
const dataDir = 'data'
const dataFile = 'settings.json'
const dataPath = `${dataDir}/${dataFile}`

// Local Method
const _createDataFile = async () => {
  try {
    await createDir(dataDir, { dir: baseDir, recursive: true })
    await writeFile({ contents: '{}', path: dataPath }, { dir: baseDir })
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
    const data = await readTextFile(dataPath, { dir: baseDir })
    return JSON.parse(data)
  } catch (e) {
    return null
  }
}

export const saveData = async (data) => {
  await writeFile({ contents: data, path: dataPath }, { dir: baseDir })
}
