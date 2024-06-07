/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  
  // basePath: '/demo/trade_front',
  // assetPrefix: '/demo/trade_front',
  basePath: '',
  assetPrefix: '',
  
  // Optional: Change links `/me` -> `/me/` and emit `/me.html` -> `/me/index.html`
  trailingSlash: true,
  // Optional: Prevent automatic `/me` -> `/me/`, instead preserve `href`
  // skipTrailingSlashRedirect: true,
 
  // Optional: Change th

  // images: {
  //   domains: ['images.dog.ceo', 'localhost'], //ここにドメインを指定
  // },
  images: {
    unoptimized: true,
  },

};

export default nextConfig;
