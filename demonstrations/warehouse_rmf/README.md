Создаем пространство

============================================ 

mkdir ~/rmf_ws/src -p

cd ~/rmf_ws/src

git clone https://github.com/robosharing/rmf.git

cd ~/rmf_ws

colcon build

============================================

запуск мира

===========================================

source ~/rmf_ws/install/setup.bash

ros2 launch rmf_demos_gz warehouse.launch.xml
===========================================
посылаем команды для движения
===========================================

ros2 run rmf_demos_tasks dispatch_patrol -p (начальная точка) (конечная точка) -n (количество повотрений) --use_sim_time
