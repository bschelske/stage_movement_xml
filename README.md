# ND2 Coordinate XML Generator 🔬
**A tool for automated linear multipoint path generation in Nikon NIS-Elements.**

---

## 📋 Purpose
The purpose of this program is to generate a coordinate list of positions for a microscope to visit using an automatic stage. It calculates a linear path between a start and end point and formats it into an XML file that Nikon NIS-Elements can read.

## 🛠️ How to Use

### 1. Nikon NIS-Elements Preparation
* Prepare the **ND Acquisition** menu (`Ctrl` + `Alt` + `N`).
* Click on the **XY tab**. 
* **Check the box** for the XY tab so that your ND acquisition will visit each point in the list.
* If there are any points already in the list, remove them by pressing the **Red X** icon.
* Ensure the **"Include Z"** box is **NOT** checked.

### 2. Identify Start and End Points
* Center the **start position** of your device using the joystick.
* Right-click the center of the image (where you would like to start the sequence) and select **"Move this Point to Center"**.
* In the ND Acquisition menu, click the **"Add" icon (Blue Plus)**.
* Repeat this process for the desired **end point** of your sequence.

### 3. Record Coordinates
For easy copy-pasting, open a `notepad.txt` and record your positions in the following format:
`x1, y1, x2, y2, steps`

> [!IMPORTANT]
> **RECORD THESE STEPS IN MILLIMETERS (mm).**

* **Example:** `8.459, -9.013, 7.495, -8.988, 3`
* **Steps:** This is the number of images that will be taken between (x1, y1) and (x2, y2). You may need to adjust this number after trial and error.

### 4. Generate the XML
1. Click on **`stage_movement.exe`**.
2. When prompted, paste your coordinates using **`Ctrl` + `V`**.
3. If you make a mistake, the program will notify you.
4. **Pro-Tip:** To repeat or edit your previous input, press the **Up Arrow** key.
5. Once valid input is entered, an `.xml` file will be saved to:
   ` .exe location > output`

### 5. Load into Nikon
1. In the Nikon ND Acquisition menu, click **"Load..."**
2. Select the `.xml` file you just generated.
3. The XY point list will now be populated with your calculated path.

