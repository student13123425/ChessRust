use std::sync::atomic::AtomicI32;
use raylib::color::Color;
use raylib::consts::MouseButton;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use crate::Aox::{Rect2D, Vec2D};
use crate::Rendering::{draw_centered_texture, draw_rounded_rect_center};
use crate::TextureMap::TextureMap;

pub struct PiceSelectMenu {
    rect:Vec<Rect2D>,
    animation:Vec<f32>
}

impl PiceSelectMenu {
    pub fn new() -> Self {
        let mut rects = Vec::new();
        let rect_w = 180;
        let rect_h = 180;
        for i in 0..4 {
            let center_x = (1000 / 2) - 300 + i * 200;
            let center_y = 1100 / 2;
            let x = center_x - (rect_w / 2);
            let y = center_y - (rect_h / 2);
            rects.push(Rect2D::new(x, y, rect_w, rect_h));
        }
        return Self {
            rect:rects,
            animation:vec![0.0,0.0,0.0,0.0]
        };
    }
    pub fn render(&self,d:&mut RaylibDrawHandle,map:&TextureMap,side:bool){
        d.draw_rectangle(0,0,9999,9999,Color::new(0,0,0,150));
        draw_rounded_rect_center(d,800.0,200.0,0.1,Vec2D::new((1000/2),(1100/2)),Color::WHITE,1.0);
        let v:u8=150;
        for i in 0..4{
            let center=Vec2D::new((1000/2)-300+i*200,(1100/2));
            draw_rounded_rect_center(d,180.0,180.0,0.1,center.clone(),Color::new(v,v,v,0),self.animation[i as usize]);
            if(!side){
                draw_centered_texture(d,&map.white_textures[(i+1) as usize],center.clone(),150,false,1.0);
            }else{
                draw_centered_texture(d,&map.black_textures[(i+1) as usize],center.clone(),150,false,1.0);
            }
        }
    }
    pub fn process_animation(&mut self,d:&mut RaylibDrawHandle) {
        let mouse_pos:Vec2D=Vec2D::new(d.get_mouse_x(),d.get_mouse_y());
        let speed:f32=3.5;
        for i in 0..4{
            if(self.rect[i].contains(mouse_pos)){
                self.animation[i]+=speed*d.get_frame_time();
            }else{
                self.animation[i]-=speed*d.get_frame_time();
            }
            if(self.animation[i]>1.0){self.animation[i]=1.0;}
            if(self.animation[i]<0.0){self.animation[i]=0.0;}
        }
    }
    pub fn get_clicks(&mut self,d:&mut RaylibDrawHandle)->i32{
        let mouse_pos:Vec2D=Vec2D::new(d.get_mouse_x(),d.get_mouse_y());
        let is_clicked=d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        for i in 0..4{
            if(self.rect[i].contains(mouse_pos)&& is_clicked){
                return  (i as i32)+1;
            }
        }
        return -1;
    }

    pub fn update(&mut self,d:&mut RaylibDrawHandle)->i32{
        self.process_animation(d);
        return self.get_clicks(d);
    }
}