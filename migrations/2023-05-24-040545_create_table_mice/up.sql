CREATE TABLE mice (
  id VARCHAR(255) PRIMARY KEY,
  brand VARCHAR(255) NOT NULL,
  model VARCHAR(255),
  width FLOAT[] NOT NULL,
  height FLOAT[] NOT NULL,
  length FLOAT NOT NULL,
  weight FLOAT NOT NULL,
  shape TEXT CHECK (shape IN ('AMBIDEXTROUS', 'ERGONOMIC')) DEFAULT 'ERGONOMIC' NOT NULL,
  wireless BOOLEAN NOT NULL,
  sensor VARCHAR(255),
  mcu VARCHAR(255),
  dpi VARCHAR(255),
  polling_rate VARCHAR(255),
  switches VARCHAR(255),
  mouse_wheel_encoder VARCHAR(255),
  material TEXT CHECK(material IN ('PLASTIC', 'MAGNESIUM')) DEFAULT 'PLASTIC' NOT NULL,
  launch_date DATE
)