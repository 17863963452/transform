fid=fopen('fai19m.dat','r');

xx=33; yy=18; zz=19; 
lat=[12:4:83];
lon=[32:4:163];
%p on 19 levels  100  150 200 250 300 350 400 450 500 550 600 650  700 750
%800 850 900 950 1000hPa
fai=fread(fid,[yy xx*zz],'float');
fai19=reshape(fai,yy,xx,zz);
fai1=fai19(:,:,9);%(500hPa)


contourf(lon,lat,fai1)
colorbar
