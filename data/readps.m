fid=fopen('psom.dat','r');

xx=33; yy=18; 
lat=[12:4:83];
lon=[32:4:163];

ps0=fread(fid,[yy xx],'float');
disp(ps0)
contourf(lon,lat,ps0)
colorbar
