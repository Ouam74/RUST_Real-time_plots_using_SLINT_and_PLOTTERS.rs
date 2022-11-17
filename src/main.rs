# Tasks:
# In Circuit :
# 1-Get the "SPICE component" --> create a "V-probe" at each port,
# 2-Create "Mag" and "Phase" plots for each port,
# 3-Export plots to ".tab" file,

# In HFSS,
# 4- import the ".tab" files into the "Design Datasets..."
# 5- Import the datasets into the "Excitations" ("Edit Sources...") like : pwl(Port1_T1_ang,Freq)

import sys
import os
import pyaedt
from pyaedt import Circuit
from pyaedt import Hfss

sys.setrecursionlimit(100_000)
print(chr(27) + "[2J")

# inputs
project_name = "D:\\5-Customer_Issues\\Infineon_Muenchen\\15-11-2022\\Q3__Segments_template_PCB_EXP3D_general_Pushed_Olivier.aedt"
cir_design_name = "1_Circuit_SPICE_LINK_TEST"
hfss_design_name = "HFSSDesign1"
comp_to_push_name = "HFSSDesign1_HFSS_Setup_1_Sweep_1_sp"
comp_to_push_id = -1000

# Delete "lock" file
pyaedt.generic.general_methods.remove_project_lock(project_name)

# Open the "Circuit" and "HFSS" sessions
# cir = pyaedt.Circuit(projectname=project_name, designname=design_name, non_graphical=False, specified_version="2022.2")
cir = pyaedt.Circuit(designname=cir_design_name, non_graphical=False, specified_version="2022.2")
h = pyaedt.Hfss(designname=hfss_design_name, non_graphical=False, specified_version="2022.2")

# Find the working directory
wd = cir.project_path

# Find the "SPICE component" --> create a "V-probe" at each port
pr = []
comp_list = cir.modeler.components.components
for id in comp_list:
    if comp_to_push_name in cir.modeler.components[id].name:
        comp_to_push_id = id
        pr = [None] * len(cir.modeler.components[id].pins)
        for i in range(len(pr)):
            print(i)
            pr[i] = cir.modeler.components.components_catalog["Probes:VPROBE"].place("bla" + str(i))
            pr[i].parameters["Name"] = cir.modeler.components[id].pins[i].name
            pr[i].pins[0].connect_to_component(cir.modeler.components[id].pins[i])
        break

# Simulate the project
cir.analyze_setup("NexximTransient")

# # Create the "mag" and "angle" spectral reports and save them as ".tab" files
for i in range(len(pr)):
# for i in range(4):
    # Create plots and save data to a file
    new_report = cir.post.reports_by_category.spectral("mag(V(Port%d_T1))" %(i+1))
    new_report.window = "Hanning"
    new_report.max_frequency = "100MHz"
    new_report.time_stop = "10us"
    new_report.create()
    new_report.edit_x_axis_scaling(units='Hz')
    cir.post.rename_report(new_report.plot_name, "Port%d_T1_mag" %(i+1))
    cir.post.export_report_to_file(wd, "Port%d_T1_mag" %(i+1), ".tab")

    new_report = cir.post.reports_by_category.spectral("ang_rad(V(Port%d_T1))" %(i+1))
    new_report.window = "Hanning"
    new_report.max_frequency = "100MHz"
    new_report.time_stop = "10us"
    new_report.create()
    new_report.edit_x_axis_scaling(units='Hz')
    new_report.edit_x_axis_scaling(linear_scaling=False)
    cir.post.rename_report(new_report.plot_name, "Port%d_T1_ang" %(i+1))
    cir.post.export_report_to_file(wd, "Port%d_T1_ang" %(i+1), ".tab")

cir.save_project()

# Get ".tab" file list
for file in os.listdir(wd):
    if file.endswith(".tab"):
        file = os.path.join(wd, file)
        name = os.path.splitext(os.path.basename(file))[0]
        h.import_dataset1d(file, dsname=name, is_project_dataset=False)

for i in range(len(pr)):
    h.edit_source(portandmode="Port%d_T1" %(i+1), powerin="pwl(Port%d_T1_mag,Freq)" %(i+1), phase="pwl(Port1_T1_ang,Freq)")

h.analyze_setup("Setup1")
# h.save_project()
