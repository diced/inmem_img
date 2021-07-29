# In Memory Image
This stores images in memory it's that simple.

# API
`/api/upload` - send multipart/form/data with a file
`/api/files/{file}` - get a files metadata stored
`/api/stats` - get stats on how much bytes are stored and the total images
`/{file}` - get a file

# Deploy
[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new/template?template=https%3A%2F%2Fgithub.com%2Fdiced%2Finmem_img&envs=AUTHORIZATION%2CRAND_LENGTH&optionalEnvs=RAND_LENGTH&AUTHORIZATIONDesc=used+to+upload+images&RAND_LENGTHDesc=the+amount+of+chars+to+generate+for+files)